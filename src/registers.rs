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

/// Default I2C address
pub const TSL2591_ADDR: u8 = 0x29;
/// 1010 0000: bits 7 and 5 for 'command normal'
pub(crate) const TSL2591_COMMAND_BIT: u8 = 0xA0;
/// Special Function Command for "Clear ALS and no persist ALS interrupt"
pub(crate) const TSL2591_CLEAR_INT: u8 = 0xE7;
/// Special Function Command for "Interrupt set - forces an interrupt"
pub(crate) const TSL2591_TEST_INT: u8 = 0xE4;
/// 1: u8 =  read/write word: u8 =  rather than byte)
pub(crate) const TSL2591_WORD_BIT: u8 = 0x20;
/// 1: u8 =  using block read/write
pub(crate) const TSL2591_BLOCK_BIT: u8 = 0x10;
/// Flag for ENABLE register to disable
pub(crate) const TSL2591_ENABLE_POWEROFF: u8 = 0x00;
/// Flag for ENABLE register to enable
pub(crate) const TSL2591_ENABLE_POWERON: u8 = 0x01;
/// ALS Enable. This field activates ALS function. Writing a one
/// activates the ALS. Writing a zero disables the ALS.
pub(crate) const TSL2591_ENABLE_AEN: u8 = 0x02;
/// ALS Interrupt Enable. When asserted permits ALS interrupts to be
/// generated, subject to the persist filter.
pub(crate) const TSL2591_ENABLE_AIEN: u8 = 0x10;
/// No Persist Interrupt Enable. When asserted NP Threshold conditions
/// will generate an interrupt, bypassing the persist filter
pub(crate) const TSL2591_ENABLE_NPIEN: u8 = 0x80;

/// TSL2591 Register map
#[repr(u8)]
pub enum Register {
    /// Enable register
    Enable = 0x00,
    /// Control register
    Control = 0x01,
    // ALS low threshold lower byte
    ThresholdAiltl = 0x04,
    /// ALS low threshold upper byte
    ThresholdAilth = 0x05,
    /// ALS high threshold lower byte
    ThresholdAihtl = 0x06,
    /// ALS high threshold upper byte
    ThresholdAihth = 0x07,
    /// No Persist ALS low threshold lower byte
    ThresholdNpailtl = 0x08,
    /// No Persist ALS low threshold higher byte
    ThresholdNpailth = 0x09,
    /// No Persist ALS high threshold lower byte
    ThresholdNpaihtl = 0x0A,
    /// No Persist ALS high threshold higher byte
    ThresholdNpainth = 0x0B,
    /// Interrupt persistence filter
    PersistFilter = 0x0C,
    /// Package Identification
    PackagePID = 0x11,
    /// Device Identification
    DeviceID = 0x12,
    /// Internal Status
    DeviceStatus = 0x13,
    /// Channel 0 data, low byte
    Chan0Low = 0x14,
    /// Channel 0 data, high byte
    Chan0High = 0x15,
    /// Channel 1 data, low byte
    Chan1Low = 0x16,
    /// Channel 1 data, hight byte
    Chan1High = 0x17,
}

/// Enumeration for the sensor integration timing
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum IntegrationTime {
    /// 100 millis
    OneHundredMS = 0x00,
    /// 200 millis
    TwoHundredMS = 0x01,
    /// 300 millis
    ThreeHundredMS = 0x02,
    /// 400 millis
    FourHundredMS = 0x03,
    /// 500 millis
    FiveHundredMS = 0x04,
    /// 600 millis
    SixHundredMS = 0x05,
}

/// Interrupt persistence filter settings
///
/// Determines how many consecutive readings outside thresholds trigger an interrupt.
///
/// # Hardware Note
/// **Bits 7-4 are reserved and must be 0.** Only bits 3-0 (values 0-15) are used.
/// This means valid persistence values range from 0x00 to 0x0F.
#[repr(u8)]
pub enum Persist {
    /// Every ALS cycle generates an interrupt
    Every = 0x00,
    /// Any value outside of threshold range
    Any = 0x01,
    /// 2 consecutive values out of range
    Two = 0x02,
    /// 3 consecutive values out of range
    Three = 0x03,
    /// 5 consecutive values out of range
    Five = 0x04,
    /// 10 consecutive values out of range
    Ten = 0x05,
    /// 15 consecutive values out of range
    Fifteen = 0x06,
    /// 20 consecutive values out of range
    Twenty = 0x07,
    /// 25 consecutive values out of range
    TwentyFive = 0x08,
    /// 30 consecutive values out of range
    Thirty = 0x09,
    /// 35 consecutive values out of range
    ThirtyFive = 0x0A,
    /// 40 consecutive values out of range
    Forty = 0x0B,
    /// 45 consecutive values out of range
    FortyFive = 0x0C,
    /// 50 consecutive values out of range
    Fifty = 0x0D,
    /// 55 consecutive values out of range
    FiftyFive = 0x0E,
    /// 60 consecutive values out of range
    Sixty = 0x0F,
}

/// Enumeration for the sensor gain
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Gain {
    /// low gain (1x)
    Low = 0x00,
    /// medium gain (25x)
    Medium = 0x10,
    /// high gain (428x)
    High = 0x20,
    /// max gain (9876x)
    Max = 0x30,
}
