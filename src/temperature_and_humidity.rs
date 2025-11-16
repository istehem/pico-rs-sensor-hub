use defmt::info;
use embassy_dht_sensor::DHTSensor;
use embassy_rp::gpio::Flex;
use embassy_time::Timer;

#[embassy_executor::task]
pub async fn task(pin: Flex<'static>) {
    let mut dht_sensor = DHTSensor::new(pin);

    loop {
        let measurement = dht_sensor.read();
        match measurement {
            Ok(measurement) => {
                info!(
                    "Temperature: {:?}, Humidity: {:?}",
                    measurement.temperature, measurement.humidity
                );
            }
            Err(_) => {
                info!("Error reading from DHT sensor");
            }
        }
        Timer::after_millis(3000).await;
    }
}
