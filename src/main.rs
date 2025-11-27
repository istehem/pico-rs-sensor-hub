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
use embedded_alloc::LlffHeap;
use {defmt_rtt as _, panic_probe as _};

mod game {
    pub mod cache;
    pub mod entities;
    pub mod error;
    pub mod player;
    pub mod tasks;
}

#[cfg(feature = "temperature")]
mod temperature_and_humidity {
    pub mod error;
    pub mod tasks;
    pub use embassy_rp::{
        gpio::Flex,
        peripherals::PIO0,
        pio::{InterruptHandler, Pio},
    };
}
#[cfg(feature = "temperature")]
pub use temperature_and_humidity::{Flex, InterruptHandler, Pio, PIO0};

const I2C_FREQUENCY: u32 = 400_000;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

bind_interrupts!(struct Irqs {
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
    #[cfg(feature = "temperature")]
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, 8 * 1024) }
    }
    let p = embassy_rp::init(Default::default());

    let mut config = I2cConfig::default();
    config.frequency = I2C_FREQUENCY;

    let led = Output::new(p.PIN_25, Level::Low);
    let sensor = Input::new(p.PIN_21, Pull::Up);
    let i2c = I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, config);

    game::tasks::spawn_tasks(&spawner, sensor, led, i2c).await;

    #[cfg(feature = "temperature")]
    {
        let pio = p.PIO0;
        let Pio {
            mut common, sm0, ..
        } = Pio::new(pio, Irqs);
        let mut pin = common.make_pio_pin(p.PIN_17);
        pin.set_pull(Pull::Up);

        temperature_and_humidity::tasks::spawn_tasks(&spawner, pin, common, sm0).await;
    }
}
