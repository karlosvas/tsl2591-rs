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

use std::fmt;

/// Errors that can occur when interacting with the TSL2591 sensor
#[derive(Debug)]
pub enum Tsl2591Error<E> {
    /// I2C communication error
    ///
    /// This wraps the error type from the underlying [`embedded_hal::i2c::I2c`] implementation.
    I2c(E),

    /// The device ID doesn't match the expected value
    ///
    /// The TSL2591 should return `0x50` when reading the `DeviceID` register.
    /// This error indicates the sensor might not be connected or is not a TSL2591.
    ///
    /// # Arguments
    /// * `0` - The actual ID read from the sensor
    InvalidDevice(u8),

    /// Saturated sensor reading
    ///
    /// The light level is too high to be represented (value reached `0xFFFF`).
    /// Try reducing gain or integration time.
    Overflow,
}

impl<E: fmt::Debug> fmt::Display for Tsl2591Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E: fmt::Debug> std::error::Error for Tsl2591Error<E> {}
