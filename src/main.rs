#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ops::DerefMut;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{self, Config as I2cConfig, I2c};
use embassy_rp::peripherals::I2C1;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_time::{Instant, Timer};
use embedded_alloc::LlffHeap;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::BinaryColor;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use ssd1306::mode::BufferedGraphicsModeAsync;
use ssd1306::prelude::I2CInterface;
use ssd1306::{
    mode::DisplayConfigAsync, rotation::DisplayRotation, size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306Async,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

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

#[derive(PartialEq)]
enum DisplayCommand {
    Blink,
    Solid,
}

type DisplayCommandChannel = Channel<NoopRawMutex, DisplayCommand, 4>;
static DISPLAY_COMMAND_CHANNEL: StaticCell<DisplayCommandChannel> = StaticCell::new();

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
    let display_command_channel = DISPLAY_COMMAND_CHANNEL.init(Channel::new());

    spawner
        .spawn(play_and_draw_task(
            display,
            roll_channel,
            display_command_channel,
        ))
        .unwrap();
    spawner
        .spawn(blink_display_task(display, display_command_channel))
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
    display_command_channel: &'static DisplayCommandChannel,
) {
    let seed = roll_channel.receive().await;
    let mut game = Game::new(SmallRng::seed_from_u64(seed));

    loop {
        if game.dice_left == NumberOfDice::Five {
            display_command_channel.send(DisplayCommand::Solid).await;
            display.lock().await.set_invert(false).await.unwrap();
        }
        let game_over = {
            let mut display = display.lock().await;
            display.set_display_on(true).await.unwrap();
            let game_over = play_and_draw(display.deref_mut(), &mut game).unwrap();
            display.flush().await.unwrap();
            game_over
        };
        if game_over {
            display_command_channel.send(DisplayCommand::Blink).await;
        }
        roll_channel.receive().await;
    }
}

#[embassy_executor::task]
async fn blink_display_task(
    display: &'static DisplayMutex,
    display_command_channel: &'static DisplayCommandChannel,
) {
    let mut invert_display = false;
    let mut display_state = DisplayCommand::Solid;

    loop {
        display_state = match display_command_channel.try_receive() {
            Ok(DisplayCommand::Blink) => DisplayCommand::Blink,
            Ok(DisplayCommand::Solid) => {
                display.lock().await.set_invert(false).await.unwrap();
                DisplayCommand::Solid
            }
            _ => display_state,
        };

        if display_state == DisplayCommand::Blink {
            display
                .lock()
                .await
                .set_invert(invert_display)
                .await
                .unwrap();
            invert_display = !invert_display;
        }

        Timer::after_millis(500).await;
    }
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
            messages::big_centered_message(score.to_string().as_str(), display)?;
        }
        game.reset();
        Ok(true)
    }
}
