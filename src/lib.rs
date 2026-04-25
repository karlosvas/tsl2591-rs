//! # tsl2591-rs
//!
//! Rust port of the [Adafruit TSL2591 Arduino Library](https://github.com/adafruit/Adafruit_TSL2591_Library),
//! originally written by KT0WN (<https://adafruit.com>).
//!
//! Platform-agnostic driver for the TSL2591 High Dynamic Range Digital Light Sensor,
//! built on [`embedded-hal`] I2C traits.
//!
//! If you find this useful, consider supporting Adafruit's open source hardware:
//! <https://www.adafruit.com/products/1980>

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
pub mod driver;
mod error;
mod registers;

pub use crate::driver::AdafruitTSL2591;
pub use crate::registers::{Gain, IntegrationTime, Persist, TSL2591_ADDR};
