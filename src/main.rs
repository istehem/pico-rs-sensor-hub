#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::borrow::BorrowMut;
use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::i2c::{self, Config as I2cConfig, I2c};
use embassy_rp::peripherals::I2C1;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
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
type RollChannel = Channel<NoopRawMutex, u64, 4>;

const I2C_FREQUENCY: u32 = 400_000;
const ONE_SECOND_IN_MUS: u64 = 1000000;

static ROLL_CHANNEL: StaticCell<RollChannel> = StaticCell::new();

static DISPLAY: Mutex<ThreadModeRawMutex, Option<Display>> = Mutex::new(None);

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

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
        DISPLAY.lock().await.borrow_mut().as_mut().unwrap(),
    )
    .unwrap();
    display.flush().await.unwrap();

    *DISPLAY.lock().await = Some(display);

    spawner.spawn(play_and_draw_task(roll_channel)).unwrap();
    spawner.spawn(blink_display_task()).unwrap();
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
async fn play_and_draw_task(roll_channel: &'static RollChannel) {
    let seed = roll_channel.receive().await;
    let mut game = Game::new(SmallRng::seed_from_u64(seed));

    loop {
        {
            let mut display_opt = DISPLAY.lock().await;
            if let Some(display) = display_opt.as_mut() {
                display.set_display_on(true).await.unwrap();
                play_and_draw(display, &mut game).unwrap();
                display.flush().await.unwrap();
            }
        }
        roll_channel.receive().await;
    }
}

#[embassy_executor::task]
async fn blink_display_task() {
    let mut display_on = false;

    loop {
        {
            let mut display_opt = DISPLAY.lock().await;
            if let Some(display) = display_opt.as_mut() {
                display.set_display_on(false).await.unwrap();
            }
        }
        display_on = !display_on;
        Timer::after_millis(500).await;
    }
}

fn play_and_draw<T>(display: &mut T, game: &mut Game) -> Result<(), DrawError<T::Error>>
where
    T: DisplayTrait,
{
    display.clear(BinaryColor::Off)?;
    if game.dice_left > NumberOfDice::Zero {
        game.roll();
        game.rolled.draw(display)?;
        info!("current score: {}", game.score());
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
    }
    Ok(())
}
