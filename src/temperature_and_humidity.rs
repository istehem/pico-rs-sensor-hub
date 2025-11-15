use am2320::Am2320;
use defmt::info;
use embassy_rp::{i2c::I2c, peripherals::I2C1};
use embassy_time::{Delay, Timer};

#[embassy_executor::task]
pub async fn task(i2c: I2c<'static, I2C1, embassy_rp::i2c::Async>) {
    let delay = Delay {};
    let mut am2320 = Am2320::new(i2c, delay);

    loop {
        let measurement = am2320.read().unwrap();
        info!("temperature is {:?}", measurement.temperature);
        Timer::after_millis(3000).await;
    }
}
