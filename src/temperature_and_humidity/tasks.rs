use defmt::info;
use embassy_dht_sensor::DHTSensor;
use embassy_executor::Spawner;
use embassy_rp::{
    peripherals::PIO0,
    pio::{Common, Pin, StateMachine},
};
use embassy_time::Timer;

use crate::temperature_and_humidity::error::FormattableDHTSensorError;

type Pio = PIO0;
type DHTStateMachine = StateMachine<'static, Pio, 0>;

pub async fn spawn_tasks(
    spawner: &Spawner,
    sensor_pin: Pin<'static, Pio>,
    common: Common<'static, Pio>,
    state_machine: DHTStateMachine,
) {
    spawner
        .spawn(read_sensor_task(sensor_pin, common, state_machine))
        .unwrap();
}

#[embassy_executor::task]
async fn read_sensor_task(
    sensor_pin: Pin<'static, Pio>,
    common: Common<'static, Pio>,
    state_machine: DHTStateMachine,
) {
    let mut dht_sensor = DHTSensor::new(sensor_pin, common, state_machine);

    loop {
        let measurement = dht_sensor.read().await;
        match measurement {
            Ok(measurement) => {
                info!(
                    "Temperature: {}, Humidity: {}",
                    measurement.temperature, measurement.humidity
                );
            }
            Err(err) => {
                info!(
                    "Error reading from DHT sensor: {}",
                    FormattableDHTSensorError::from(err)
                );
            }
        }
        Timer::after_millis(3000).await;
    }
}
