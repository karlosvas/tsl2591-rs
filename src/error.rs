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

pub enum Error<E> {
    // Error with I2C bus — wrappea el error del trait embedded-hal
    I2c(E),
    // The device ID doesn't match the expected value
    InvalidDevice(u8),
    // Saturated sensor reading — the value is too high to be represented
    Overflow,
}
