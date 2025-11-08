#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use defmt::info;
use display_interface::DisplayError;
use embassy_executor::Spawner;
use embassy_futures::select::{select, select3, Either, Either3};
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    i2c::{self, Config as I2cConfig, I2c},
    peripherals::I2C1,
};
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex, channel::Channel, mutex::Mutex, pubsub::PubSubChannel,
};
use embassy_time::{Instant, Timer};
use embedded_alloc::LlffHeap;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use ssd1306::{
    mode::{BufferedGraphicsModeAsync, DisplayConfigAsync},
    prelude::I2CInterface,
    rotation::DisplayRotation,
    size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306Async,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use embedded_graphics_framebuf::FrameBuf;

use game_logic::two_four_eighteen::{Game, NumberOfDice};
use pico_display::aliases::Display as DisplayTrait;
use pico_display::messages;

mod error;
use crate::error::DrawError;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

type Display = Ssd1306Async<
    I2CInterface<I2c<'static, I2C1, i2c::Async>>,
    DisplaySize128x64,
    BufferedGraphicsModeAsync<DisplaySize128x64>,
>;
type DisplayMutex = Mutex<NoopRawMutex, Display>;
type RollChannel = Channel<NoopRawMutex, u64, 4>;

const I2C_FREQUENCY: u32 = 400_000;
const ONE_SECOND_IN_MUS: u64 = 1000000;

static ROLL_CHANNEL: StaticCell<RollChannel> = StaticCell::new();
static DISPLAY: StaticCell<DisplayMutex> = StaticCell::new();

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

#[derive(PartialEq, Clone)]
enum DisplayState {
    Blink,
    Solid,
}

type DisplayStateChannel = PubSubChannel<NoopRawMutex, DisplayState, 4, 2, 1>;
static DISPLAY_STATE_CHANNEL: StaticCell<DisplayStateChannel> = StaticCell::new();

type DisplayBuffer = [BinaryColor; 8192];
type DisplayBufferChannel = Channel<NoopRawMutex, DisplayBuffer, 1>;
static DISPLAY_BUFFER_CHANNEL: StaticCell<DisplayBufferChannel> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }
    let p = embassy_rp::init(Default::default());

    let roll_channel = ROLL_CHANNEL.init(Channel::new());

    let led = Output::new(p.PIN_25, Level::Low);
    let sensor = Input::new(p.PIN_21, Pull::Up);

    spawner
        .spawn(break_beam_roller_task(sensor, led, roll_channel))
        .unwrap();

    let mut config = I2cConfig::default();
    config.frequency = I2C_FREQUENCY;
    let i2c = I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, config);
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();
    display.clear(BinaryColor::Off).unwrap();
    messages::medium_sized_centered_message(
        "Break the beam for\n at least one second\n to start the game.",
        &mut display,
    )
    .unwrap();
    display.flush().await.unwrap();

    let display = DISPLAY.init(Mutex::new(display));
    let display_state_channel = DISPLAY_STATE_CHANNEL.init(PubSubChannel::new());

    let display_buffer_channel = DISPLAY_BUFFER_CHANNEL.init(Channel::new());
    spawner
        .spawn(play_and_draw_task(
            display,
            roll_channel,
            display_state_channel,
            display_buffer_channel,
        ))
        .unwrap();
    spawner
        .spawn(display_state_handler_task(display, display_state_channel))
        .unwrap();

    spawner
        .spawn(display_toggler_task(
            display,
            display_state_channel,
            display_buffer_channel,
        ))
        .unwrap();
}

#[embassy_executor::task]
async fn break_beam_roller_task(
    mut sensor: Input<'static>,
    mut led: Output<'static>,
    roll_channel: &'static RollChannel,
) {
    let mut seed: Option<u64> = None;
    let mut beam_broken_at: Option<Instant> = None;

    loop {
        sensor.wait_for_any_edge().await;
        if sensor.is_high() {
            led.set_high();

            if let (Some(beam_broken_at), None) = (beam_broken_at, seed) {
                let duration = beam_broken_at.elapsed().as_micros();
                if duration > ONE_SECOND_IN_MUS {
                    roll_channel.send(duration).await;
                    seed = Some(duration);
                }
                info!("Beam broken for {} mus.", duration);
            }
        } else {
            led.set_low();

            if let Some(seed) = seed {
                roll_channel.send(seed).await;
            } else {
                beam_broken_at = Some(Instant::now());
            }
        }
        info!("Edge detected, level: {}", sensor.is_high());
    }
}

#[embassy_executor::task]
async fn play_and_draw_task(
    display: &'static DisplayMutex,
    roll_channel: &'static RollChannel,
    display_state_channel: &'static DisplayStateChannel,
    display_buffer_channel: &'static DisplayBufferChannel,
) {
    let seed = roll_channel.receive().await;
    let mut game = Game::new(SmallRng::seed_from_u64(seed));
    let mut buffer = [BinaryColor::Off; 8192];

    let display_state_publisher = display_state_channel.publisher().unwrap();

    loop {
        if game.dice_left == NumberOfDice::Five {
            display_state_publisher.publish(DisplayState::Solid).await;
        }
        let game_over = {
            let mut framebuffer = FrameBuf::new(&mut buffer, 128, 64);
            let game_over = play_and_draw(&mut framebuffer, &mut game).unwrap();

            let mut display = display.lock().await;
            display.draw_iter(framebuffer.into_iter()).unwrap();
            display.flush().await.unwrap();
            game_over
        };
        if game_over {
            display_state_publisher.publish(DisplayState::Blink).await;
            display_buffer_channel.send(buffer).await;
        }
        roll_channel.receive().await;
    }
}

#[derive(PartialEq)]
enum ToggleState {
    YouWin,
    Fish,
    Result,
}

#[embassy_executor::task]
async fn display_toggler_task(
    display: &'static DisplayMutex,
    display_state_channel: &'static DisplayStateChannel,
    display_buffer_channel: &'static DisplayBufferChannel,
) {
    let mut toggle_state = ToggleState::YouWin;
    let mut display_state = DisplayState::Solid;

    let buffer = [BinaryColor::Off; 8192];
    let mut you_win_framebuffer = FrameBuf::new(buffer, 128, 64);
    let mut fish_framebuffer = FrameBuf::new(buffer, 128, 64);
    let mut game_framebuffer = FrameBuf::new(buffer, 128, 64);

    messages::big_centered_message("18!\nYou Win!", &mut you_win_framebuffer).unwrap();
    messages::big_centered_message("Fish!", &mut fish_framebuffer).unwrap();

    let mut display_state_subscriber = display_state_channel.subscriber().unwrap();

    loop {
        match select3(
            Timer::after_millis(2000),
            display_state_subscriber.next_message_pure(),
            display_buffer_channel.receive(),
        )
        .await
        {
            Either3::First(_) => {
                if display_state == DisplayState::Blink {
                    let mut display = display.lock().await;
                    if toggle_state == ToggleState::Fish {
                        display.draw_iter(you_win_framebuffer.into_iter()).unwrap();
                        toggle_state = ToggleState::Result;
                    } else if toggle_state == ToggleState::YouWin {
                        display.draw_iter(fish_framebuffer.into_iter()).unwrap();
                        toggle_state = ToggleState::Result;
                    } else if toggle_state == ToggleState::Result {
                        display.draw_iter(game_framebuffer.into_iter()).unwrap();
                        toggle_state = ToggleState::Fish;
                    }
                    display.flush().await.unwrap();
                }
            }
            Either3::Second(state) => {
                display_state = state;
            }
            Either3::Third(buffer) => {
                game_framebuffer = FrameBuf::new(buffer, 128, 64);
            }
        }
    }
}

#[embassy_executor::task]
async fn display_state_handler_task(
    display: &'static DisplayMutex,
    display_state_channel: &'static DisplayStateChannel,
) {
    let mut invert_display = false;
    let mut display_state = DisplayState::Solid;
    let mut display_state_subscriber = display_state_channel.subscriber().unwrap();

    loop {
        match select(
            Timer::after_millis(1000),
            display_state_subscriber.next_message_pure(),
        )
        .await
        {
            Either::First(_) => {
                if display_state == DisplayState::Blink {
                    invert_display = !invert_display;
                    set_invert_display(display, invert_display).await.unwrap();
                }
            }
            Either::Second(state) => {
                display_state = state;
                invert_display = display_state != DisplayState::Solid;
                set_invert_display(display, invert_display).await.unwrap();
            }
        }
    }
}

async fn set_invert_display(display: &DisplayMutex, invert: bool) -> Result<(), DisplayError> {
    display.lock().await.set_invert(invert).await
}

fn play_and_draw<T>(display: &mut T, game: &mut Game) -> Result<bool, DrawError<T::Error>>
where
    T: DisplayTrait,
{
    display.clear(BinaryColor::Off)?;
    if game.dice_left > NumberOfDice::Zero {
        game.roll();
        game.rolled.draw(display)?;
        info!("current score: {}", game.score());
        Ok(false)
    } else {
        let mut picked: Vec<String> = game
            .picked
            .dice
            .iter()
            .map(|die| die.value.as_u8().to_string())
            .collect();
        picked.sort();
        info!("picked: {}", picked.join(",").as_str());
        let score = game.score();
        info!("final score: {}", score);
        if game.has_fish() {
            messages::big_centered_message("Fish!", display)?;
        } else if game.has_won() {
            messages::big_centered_message("18!\nYou Win!", display)?;
        } else {
            game.picked.draw(display)?;
        }
        game.reset();
        Ok(true)
    }
}
