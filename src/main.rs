#![no_std]
#![no_main]

extern crate alloc;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull},
    i2c::{self, Config as I2cConfig, I2c},
    peripherals::I2C1,
};
use embassy_sync::{channel::Channel, mutex::Mutex};
use embedded_alloc::LlffHeap;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use ssd1306::{
    mode::DisplayConfigAsync, rotation::DisplayRotation, size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306Async,
};
use {defmt_rtt as _, panic_probe as _};

use pico_display::messages;

mod error;
use crate::error::DrawError;
mod cache;
mod entities;
mod game;
mod player;
mod temperature_and_humidity;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

const I2C_FREQUENCY: u32 = 400_000;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }
    let p = embassy_rp::init(Default::default());

    let roll_channel = game::ROLL_CHANNEL.init(Channel::new());

    let led = Output::new(p.PIN_25, Level::Low);
    let sensor = Input::new(p.PIN_21, Pull::Up);

    spawner
        .spawn(game::break_beam_roller_task(sensor, led, roll_channel))
        .unwrap();

    #[cfg(not(feature = "temperature"))]
    {
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

        let display = game::DISPLAY.init(Mutex::new(display));
        let display_state_channel = game::DISPLAY_STATE_CHANNEL.init(Channel::new());

        let game_state_channel = game::GAME_STATE_CHANNEL.init(Channel::new());
        spawner
            .spawn(game::play_and_draw_task(
                display,
                roll_channel,
                game_state_channel,
            ))
            .unwrap();
        spawner
            .spawn(game::display_state_handler_task(
                display,
                display_state_channel,
            ))
            .unwrap();
        spawner
            .spawn(game::display_animations_task(
                display,
                game_state_channel,
                display_state_channel,
            ))
            .unwrap();
    }

    #[cfg(feature = "temperature")]
    {
        let config = I2cConfig::default();
        let i2c = I2c::new_async(p.I2C1, p.PIN_11, p.PIN_10, Irqs, config);

        spawner.spawn(temperature_and_humidity::task(i2c)).unwrap();
    }
}
