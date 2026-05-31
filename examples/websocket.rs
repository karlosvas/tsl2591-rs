//! This example requires a Raspberry Pi with a TSL2591 sensor connected via I2C.
//! Run with: cargo run --example websocket --features serde --target aarch64-unknown-linux-gnu

use {
    dotenvy,
    futures_util::SinkExt,
    linux_embedded_hal::{Delay, I2cdev},
    std::time::Duration,
    tokio,
    tokio_tungstenite::{connect_async, tungstenite::Message},
    tsl2591_rs::{AdafruitTSL2591, Gain, IntegrationTime, TSL2591_ADDR, driver::SensorReading},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let i2c: I2cdev = I2cdev::new("/dev/i2c-1")?;
    let mut sensor: AdafruitTSL2591<I2cdev, Delay> = AdafruitTSL2591::new(
        i2c,
        Delay,
        IntegrationTime::OneHundredMS,
        Gain::Medium,
        TSL2591_ADDR,
    );
    sensor.begin()?;

    let ip_host: String = std::env::var("IP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let url: String = format!("ws://{}:3000", ip_host);

    let (mut socket, _) = connect_async(url).await?;

    loop {
        let sensor_reading: SensorReading = sensor.get_event()?;

        let bytes: Vec<u8> =
            bincode::serde::encode_to_vec(&sensor_reading, bincode::config::standard())?;

        let msg: Message = Message::Binary(bytes.into());

        socket.send(msg).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
