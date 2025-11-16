use defmt::info;
use embassy_dht_sensor::DHTSensor;
use embassy_executor::Spawner;
use embassy_rp::gpio::Flex;
use embassy_time::Timer;

pub async fn spawn_tasks(spawner: &Spawner, sensor_pin: Flex<'static>) {
    spawner.spawn(read_sensor_task(sensor_pin)).unwrap();
}

#[embassy_executor::task]
async fn read_sensor_task(sensor_pin: Flex<'static>) {
    let mut dht_sensor = DHTSensor::new(sensor_pin);

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
