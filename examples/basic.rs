use linux_embedded_hal::{Delay, I2cdev};
use std::time::{Duration, Instant};
use tsl2591_rs::{AdafruitTSL2591, Gain, IntegrationTime, TSL2591_ADDR, driver::SensorReading};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i2c = I2cdev::new("/dev/i2c-1")?;
    let mut delay = Delay;
    let mut sensor = AdafruitTSL2591::new(
        i2c,
        delay,
        IntegrationTime::OneHundredMS,
        Gain::Medium,
        TSL2591_ADDR,
    );

    sensor.begin()?;

    let start: Instant = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(60) {
        let reading: SensorReading = sensor.get_event()?;
        println!("Lux: {:.2}", reading.lux);
        println!("Full spectrum: {}", reading.full_spectrum);
        println!("Infrared: {}", reading.infrared);
    }

    Ok(())
}
