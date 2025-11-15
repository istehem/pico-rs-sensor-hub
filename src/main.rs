#![no_std]
#![no_main]

extern crate alloc;

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    i2c::{self, Config as I2cConfig, I2c},
    peripherals::I2C1,
};
use embedded_alloc::LlffHeap;
use {defmt_rtt as _, panic_probe as _};

#[cfg(not(feature = "temperature"))]
mod game {
    pub use ::embassy_rp::gpio::{Input, Level, Output, Pull};

    pub mod cache;
    pub mod entities;
    pub mod error;
    pub mod player;
    pub mod tasks;

    pub const I2C_FREQUENCY: u32 = 400_000;
}
#[cfg(not(feature = "temperature"))]
pub use game::{cache, entities, error, player, Input, Level, Output, Pull, I2C_FREQUENCY};

#[cfg(feature = "temperature")]
mod temperature_and_humidity;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }
    let p = embassy_rp::init(Default::default());

    #[cfg(not(feature = "temperature"))]
    {
        let mut config = I2cConfig::default();
        config.frequency = I2C_FREQUENCY;

        let led = Output::new(p.PIN_25, Level::Low);
        let sensor = Input::new(p.PIN_21, Pull::Up);
        let i2c = I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, config);

        game::tasks::spawn_tasks(&spawner, sensor, led, i2c).await
    }

    #[cfg(feature = "temperature")]
    {
        let config = I2cConfig::default();
        let i2c = I2c::new_async(p.I2C1, p.PIN_11, p.PIN_10, Irqs, config);

        spawner.spawn(temperature_and_humidity::task(i2c)).unwrap();
    }
}
