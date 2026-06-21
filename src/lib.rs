//! # tsl2591-rs
//!
//! Rust port of the [Adafruit TSL2591 Arduino Library](https://github.com/adafruit/Adafruit_TSL2591_Library),
//! originally written by KT0WN (<https://adafruit.com>).
//!
//! Platform-agnostic driver for the TSL2591 High Dynamic Range Digital Light Sensor,
//! built on [`embedded-hal`](https://docs.rs/embedded-hal) I2C traits.
//!
//! If you find this useful, consider supporting Adafruit's open source hardware:
//! <https://www.adafruit.com/products/1980>
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use tsl2591_rs::{AdafruitTSL2591, Gain, IntegrationTime, TSL2591_ADDR};
//!
//! // Provide your platform's I2C and Delay implementations,
//! // e.g. with linux_embedded_hal on Raspberry Pi:
//! //
//! // let i2c = linux_embedded_hal::I2cdev::new("/dev/i2c-1").unwrap();
//! // let mut sensor = AdafruitTSL2591::new(
//! //     i2c, linux_embedded_hal::Delay,
//! //     IntegrationTime::OneHundredMS,
//! //     Gain::Medium,
//! //     TSL2591_ADDR,
//! // );
//! // sensor.begin().unwrap();
//! // let reading = sensor.get_event().unwrap();
//! // println!("Lux: {:.2}", reading.lux);
//! ```
#![no_std]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
pub mod driver;
mod error;
mod registers;
pub use crate::driver::AdafruitTSL2591;
pub use crate::registers::{Gain, IntegrationTime, Persist, Register, TSL2591_ADDR};
