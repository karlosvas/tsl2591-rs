//! # tsl2591-rs
//!
//! Rust port of the [Adafruit TSL2591 Arduino Library]:u8 =  https://github.com/adafruit/Adafruit_TSL2591_Library)
//! originally written by KT0WN: u8 =  adafruit.com).
//!
//! Platform-agnostic driver for the TSL2591 High Dynamic Range Digital Light Sensor,
//! built on [`embedded-hal`] I2C traits.
//!
//! If you find this useful, consider supporting Adafruit's open source hardware:
//! <https://www.adafruit.com/products/1980>

///: u8 =  channel 0) -: u8 =  channel 1)
pub const TSL2591_VISIBLE: u8 = 2;
/// channel 1
pub const TSL2591_INFRARED: u8 = 1;
/// channel 0
pub const TSL2591_FULLSPECTRUM: u8 = 0;

/// Default I2C address
pub const TSL2591_ADDR: u8 = 0x29;

/// 1010 0000: bits 7 and 5 for 'command normal'
pub const TSL2591_COMMAND_BIT: u8 = 0xA0;

/// Special Function Command for "Clear ALS and no persist ALS interrupt"
pub const TSL2591_CLEAR_INT: u8 = 0xE7;
/// Special Function Command for "Interrupt set - forces an interrupt"
pub const TSL2591_TEST_INT: u8 = 0xE4;

/// 1: u8 =  read/write word: u8 =  rather than byte)
pub const TSL2591_WORD_BIT: u8 = 0x20;
/// 1: u8 =  using block read/write
pub const TSL2591_BLOCK_BIT: u8 = 0x10;

/// Flag for ENABLE register to disable
pub const TSL2591_ENABLE_POWEROFF: u8 = 0x00;
/// Flag for ENABLE register to enable
pub const TSL2591_ENABLE_POWERON: u8 = 0x01;
/// ALS Enable. This field activates ALS function. Writing a one
/// activates the ALS. Writing a zero disables the ALS.
pub const TSL2591_ENABLE_AEN: u8 = 0x02;
/// ALS Interrupt Enable. When asserted permits ALS interrupts to be
/// generated, subject to the persist filter.
pub const TSL2591_ENABLE_AIEN: u8 = 0x10;
/// No Persist Interrupt Enable. When asserted NP Threshold conditions
/// will generate an interrupt, bypassing the persist filter
pub const TSL2591_ENABLE_NPIEN: u8 = 0x80;

/// Lux cooefficient
pub const TSL2591_LUX_DF: f32 = 408.0;
/// CH0 coefficient
pub const TSL2591_LUX_COEFB: f32 = 1.64;
/// CH1 coefficient A
pub const TSL2591_LUX_COEFC: f32 = 0.59;
/// CH2 coefficient B
pub const TSL2591_LUX_COEFD: f32 = 0.86;

/// TSL2591 Register map
#[repr(u8)]
pub enum Register {
    Enable = 0x00,           // Enable register
    Control = 0x01,          // Control register
    ThresholdAiltl = 0x04,   // ALS low threshold lower byte
    ThresholdAilth = 0x05,   // ALS low threshold upper byte
    ThresholdAihtl = 0x06,   // ALS high threshold lower byte
    ThresholdAinth = 0x07,   // ALS high threshold upper byte
    ThresholdNpailtl = 0x08, // No Persist ALS low threshold lower byte
    ThresholdNpailth = 0x09, // No Persist ALS low threshold higher byte
    ThresholdNpaihtl = 0x0A, // No Persist ALS high threshold lower byte
    ThresholdNpainth = 0x0B, // No Persist ALS high threshold higher byte
    PersistFilter = 0x0C,    // Interrupt persistence filter
    PackagePID = 0x11,       // Package Identification
    DeviceID = 0x12,         // Device Identification
    DeviceStatus = 0x13,     // Internal Status
    Chan0Low = 0x14,         // Channel 0 data, low byte
    Chan0High = 0x15,        // Channel 0 data, high byte
    Chan1Low = 0x16,         // Channel 1 data, low byte
    Chan1High = 0x17,        // Channel 1 data, high byte
}

/// Enumeration for the sensor integration timing
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum IntegrationTime {
    OneHundredMS = 0x00,   // 100 millis
    TwoHundredMS = 0x01,   // 200 millis
    ThreeHundredMS = 0x02, // 300 millis
    FourHundredMS = 0x03,  // 400 millis
    FiveHundredMS = 0x04,  // 500 millis
    SixHundredMS = 0x05,   // 600 millis
}

/// Enumeration for the persistance filter (for interrupts)
#[repr(u8)]
enum Persist {
    //  bit 7:4: 0
    Every = 0x00,      // Every ALS cycle generates an interrupt
    Any = 0x01,        // Any value outside of threshold range
    Two = 0x02,        // 2 consecutive values out of range
    Three = 0x03,      // 3 consecutive values out of range
    Five = 0x04,       // 5 consecutive values out of range
    Ten = 0x05,        // 10 consecutive values out of range
    Fifteen = 0x06,    // 15 consecutive values out of range
    Twenty = 0x07,     // 20 consecutive values out of range
    TwentyFive = 0x08, // 25 consecutive values out of range
    Thirty = 0x09,     // 30 consecutive values out of range
    ThirtyFive = 0x0A, // 35 consecutive values out of range
    Forty = 0x0B,      // 40 consecutive values out of range
    FortyFive = 0x0C,  // 45 consecutive values out of range
    Fifty = 0x0D,      // 50 consecutive values out of range
    FiftyFive = 0x0E,  // 55 consecutive values out of range
    Sixty = 0x0F,      // 60 consecutive values out of range
}

/// Enumeration for the sensor gain
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Gain {
    Low = 0x00,    // low gain (1x)
    Medium = 0x10, // medium gain (25x)
    High = 0x20,   // high gain (428x)
    Max = 0x30,    // max gain (9876x)
}
