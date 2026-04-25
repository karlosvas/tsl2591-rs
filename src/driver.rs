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

pub use crate::error::Tsl2591Error;
use crate::registers::{Gain, IntegrationTime, Persist, Register};
use embedded_hal::delay::{self, DelayNs};
use embedded_hal::i2c::I2c;

const TSL2591_FULLSPECTRUM: u8 = 0;
const TSL2591_INFRARED: u8 = 1;
const TSL2591_VISIBLE: u8 = 2;

/// Channel selection for reading specific light components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
    /// Channel 0 - Full spectrum (visible + IR)
    FullSpectrum = TSL2591_FULLSPECTRUM,
    /// Channel 1 - Infrared only
    Infrared = TSL2591_INFRARED,
    /// Calculated visible light (Full spectrum - IR)
    Visible = TSL2591_VISIBLE,
}

/// Lux cooefficient
pub(crate) const TSL2591_LUX_DF: f32 = 408.0;
/// CH0 coefficient
pub(crate) const TSL2591_LUX_COEFB: f32 = 1.64;
/// CH1 coefficient A
pub(crate) const TSL2591_LUX_COEFC: f32 = 0.59;
/// CH2 coefficient B
pub(crate) const TSL2591_LUX_COEFD: f32 = 0.86;

/// Main driver structure for the TSL2591 light sensor
///
/// This struct holds the I2C bus, sensor configuration, and state.
/// It is generic over any I2C implementation that implements [`embedded_hal::i2c::I2c`].
pub struct AdafruitTSL2591<I2C, D> {
    /// I2C bus instance
    i2c: I2C,
    /// Delay provider for timing
    delay: D,
    /// Sensor identifier (user-assignable)
    sensor_id: i32,
    /// Integration time setting
    integration: IntegrationTime,
    /// Gain setting
    gain: Gain,
    /// I2C address of the sensor (default 0x29)
    addr: u8,
    /// Whether the sensor has been successfully initialized
    initialized: bool,
}

/// Light sensor reading containing lux and raw channel values
///
/// Returned by [`get_event`](AdafruitTSL2591::get_event) after a successful measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorReading {
    /// Calculated illuminance in lux
    pub lux: f32,
    /// Raw full spectrum value (channel 0: visible + infrared)
    pub full_spectrum: u16,
    /// Raw infrared value (channel 1: infrared only)
    pub infrared: u16,
}

/// Type of physical sensor
///
/// Used in [`SensorInfo`] to identify what the sensor measures.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorType {
    /// Light sensor (measures illuminance in lux)
    Light,
}

#[derive(Debug, Clone)]
/// Heredated data from abstract Adafruit_Sensor
pub struct SensorInfo {
    /// Name of sensor device
    pub name: &'static str,
    /// Actual version
    pub version: u8,
    /// Identifier
    pub sensor_id: i32,
    /// Type of magnitude of sensor
    pub sensor_type: SensorType,
    /// Minimum delay between readings, in microseconds. `0` means no restriction.
    pub min_delay: u32,
    /// Maximum measurable value, in lux.
    pub max_value: f32,
    /// Minimum measurable value, in lux.
    pub min_value: f32,
    /// Smallest detectable change, in lux.
    pub resolution: f32,
}

impl<I2C: I2c, D: DelayNs> AdafruitTSL2591<I2C, D> {
    /// Constructor of [`AdafruitTSL2591`]
    pub fn new(i2c: I2C, delay: D, integration: IntegrationTime, gain: Gain, addr: u8) -> Self {
        AdafruitTSL2591 {
            i2c,
            delay,
            sensor_id: -1,
            integration,
            gain,
            addr,
            initialized: false,
        }
    }

    fn get_gain(&self) -> Gain {
        return self.gain;
    }

    fn set_gain(&mut self, new_gain: Gain) {
        self.enable();

        self.gain = new_gain;

        let integration: u8 = self.integration as u8;
        let gain: u8 = self.gain as u8;
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::Control as u8,
            Some(integration | gain),
        );

        self.disable();
    }

    fn get_timing(&self) -> IntegrationTime {
        return self.integration;
    }

    fn set_timing(&mut self, new_integration: IntegrationTime) {
        self.enable();

        self.integration = new_integration;

        let integration: u8 = self.integration as u8;
        let gain: u8 = self.gain as u8;
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::Control as u8,
            Some(integration as u8 | gain as u8),
        );

        self.disable();
    }

    fn get_addr(&self) -> u8 {
        return self.addr;
    }

    fn set_addr(&mut self, addr: u8) {
        self.addr = addr;
    }

    fn write8(&mut self, register: u8, value_optional: Option<u8>) {
        match value_optional {
            Some(value) => self.i2c.write(self.addr, &[register, value]),
            None => self.i2c.write(self.addr, &[register]),
        };
    }

    fn read8(&mut self, register: u8) -> u8 {
        let write_buf: [u8; 1] = [register];
        let mut read_buf: [u8; 1] = [0u8; 1];

        self.i2c.write_read(self.addr, &write_buf, &mut read_buf);

        read_buf[0]
    }

    fn read16(&mut self, register: u8) -> u16 {
        let write_buf: [u8; 1] = [register];
        let mut read_buf: [u8; 2] = [0u8; 2];

        self.i2c.write_read(self.addr, &write_buf, &mut read_buf);

        (read_buf[1] as u16) << 8 | (read_buf[0] as u16)
    }

    /// Enables the chip, so it's ready to take readings
    fn enable(&mut self) {
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::Enable as u8,
            Some(
                crate::registers::TSL2591_ENABLE_POWERON
                    | crate::registers::TSL2591_ENABLE_AEN
                    | crate::registers::TSL2591_ENABLE_AIEN
                    | crate::registers::TSL2591_ENABLE_NPIEN,
            ),
        );
    }

    /// Disables the chip, so it's in power down mode
    fn disable(&mut self) {
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::Enable as u8,
            Some(crate::registers::TSL2591_ENABLE_POWEROFF),
        );
    }

    /// Setups the I2C interface and hardware, identifies if chip is found
    /// addr The I2C adress of the sensor (Default 0x29)
    /// True if a TSL2591 is found, false on any failure
    fn begin(&mut self) -> Result<(), Tsl2591Error<I2C::Error>> {
        let id: u8 = self.read8(crate::registers::TSL2591_COMMAND_BIT | Register::DeviceID as u8);

        if id != 0x50 {
            return Err(Tsl2591Error::InvalidDevice(id));
        }

        self.initialized = true;

        let integration = self.integration;
        self.set_timing(integration);

        let gain: Gain = self.gain;
        self.set_gain(gain);

        self.disable();

        Ok(())
    }

    /// Calculates the visible Lux based on the two light sensors
    /// ch0 Data from channel 0 (IR+Visible)
    /// ch1 Data from channel 1 (IR)
    /// returns Lux, based on AMS coefficients (or < 0 if overflow)
    fn calculate_lux(&mut self, ch0: u16, ch1: u16) -> Result<f32, Tsl2591Error<I2C::Error>> {
        if (ch0 == 0xFFFF) || (ch1 == 0xFFFF) {
            return Err(Tsl2591Error::Overflow);
        }

        // Note: This algorithm is based on preliminary coefficients
        // provided by AMS and may need to be updated in the future
        let atime: f32 = match self.get_timing() {
            IntegrationTime::OneHundredMS => 100.0,
            IntegrationTime::TwoHundredMS => 200.0,
            IntegrationTime::ThreeHundredMS => 300.0,
            IntegrationTime::FourHundredMS => 400.0,
            IntegrationTime::FiveHundredMS => 500.0,
            IntegrationTime::SixHundredMS => 600.0,
        };

        let again: f32 = match self.get_gain() {
            Gain::Low => 1.0,
            Gain::Medium => 25.0,
            Gain::High => 428.0,
            Gain::Max => 9876.0,
        };

        // cpl = (ATIME * AGAIN) / DF
        let cpl: f32 = (atime * again) / TSL2591_LUX_DF;

        // Original lux calculation (for reference sake)
        // float lux1 = ( (float)ch0 - (TSL2591_LUX_COEFB * (float)ch1) ) / cpl;
        // float lux2 = ( ( TSL2591_LUX_COEFC * (float)ch0 ) - ( TSL2591_LUX_COEFD *
        // (float)ch1 ) ) / cpl; lux = lux1 > lux2 ? lux1 : lux2;

        // Alternate lux calculation 1
        // See: https://github.com/adafruit/Adafruit_TSL2591_Library/issues/14
        let lux: f32 = (ch0 as f32 - ch1 as f32) * (1.0 - (ch1 as f32 / ch0 as f32)) / cpl;

        // Alternate lux calculation 2
        // lux = ( (float)ch0 - ( 1.7F * (float)ch1 ) ) / cpl;

        // Signal I2C had no errors
        Ok(lux)
    }

    /// Reads the raw data from both light channels
    /// 32-bit raw count where high word is IR, low word is IR+Visible
    fn get_full_luminosity(&mut self) -> u32 {
        self.enable();

        // Wait x ms for ADC to complete
        let cycles: u8 = self.integration as u8 + 1;
        for _ in 0..cycles {
            self.delay.delay_ms(120);
        }

        // Empaqueted 32-bit result (lower 16 bits are full spectrum, upper 16 bits are infrared)
        let mut full_spectrum: u32;
        let infrared: u16;
        full_spectrum =
            self.read16(crate::registers::TSL2591_COMMAND_BIT | Register::Chan0Low as u8) as u32;
        infrared = self.read16(crate::registers::TSL2591_COMMAND_BIT | Register::Chan1Low as u8);

        full_spectrum |= (infrared as u32) << 16;

        self.disable();

        full_spectrum
    }

    /// Reads the raw data from the channel
    /// channel Can be 0 (IR+Visible, 1 (IR) or 2 (Visible only)
    /// returns 16-bit raw count, or 0 if channel is invalid
    fn get_luminosity(&mut self, channel: u8) -> u16 {
        let full_luminosity: u32 = self.get_full_luminosity();

        match channel {
            TSL2591_FULLSPECTRUM => {
                // Reads two byte value from channel 0 (full spectrum)
                (full_luminosity & 0xFFFF) as u16
            }
            TSL2591_INFRARED => {
                // Reads two byte value from channel 1 (infrared)
                (full_luminosity >> 16) as u16
            }
            TSL2591_VISIBLE => {
                // Reads all and subtracts out just the visible!
                ((full_luminosity & 0xFFFF) - (full_luminosity >> 16)) as u16
            }
            _ => 0, // unknown channel!
        }
    }

    /// Set up the interrupt to go off when light level is outside the
    /// lower/upper range.
    /// param  lowerThreshold Raw light data reading level that is the lower value
    /// threshold for interrupt
    /// param  upperThreshold Raw light data reading level that is the higher value
    /// threshold for interrupt
    /// param  persist How many counts we must be outside range for interrupt to
    /// fire, default is any single value
    fn register_interrupt(&mut self, lower_threshold: u16, upper_threshold: u16, persist: Persist) {
        self.enable();

        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::PersistFilter as u8,
            Some(persist as u8),
        );
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::ThresholdAiltl as u8,
            Some(lower_threshold as u8),
        );
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::ThresholdAilth as u8,
            Some((lower_threshold >> 8) as u8),
        );
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::ThresholdAihtl as u8,
            Some(upper_threshold as u8),
        );
        self.write8(
            crate::registers::TSL2591_COMMAND_BIT | Register::ThresholdAihth as u8,
            Some((upper_threshold >> 8) as u8),
        );

        self.disable();
    }

    fn clear_interrupt(&mut self) {
        self.enable();
        self.write8(crate::registers::TSL2591_CLEAR_INT, None);
        self.disable();
    }

    /// Gets the most recent sensor event from the hardware status register.
    /// return Sensor status as a byte. Bit 0 is ALS Valid. Bit 4 is ALS Interrupt.
    /// Bit 5 is No-persist Interrupt.
    fn get_status(&mut self) -> u8 {
        self.enable();

        let x: u8 =
            self.read8(crate::registers::TSL2591_COMMAND_BIT | Register::DeviceStatus as u8);

        self.disable();

        x
    }

    /// Gets the most recent sensor event
    // @param  event Pointer to Adafruit_Sensor sensors_event_t object that will be
    // filled with sensor data
    // @return True on success, False on failure
    fn get_event(&mut self) -> Result<SensorReading, Tsl2591Error<I2C::Error>> {
        self.get_full_luminosity();
        // Early silicon seems to have issues when there is a sudden jump in */
        // light levels. :( To work around this for now sample the sensor 2x */
        let lum: u32 = self.get_full_luminosity();

        let ir: u16 = (lum >> 16) as u16;
        let full: u16 = (lum & 0xFFFF) as u16;

        Ok(SensorReading {
            lux: self.calculate_lux(full, ir)?,
            infrared: ir,
            full_spectrum: full,
        })
    }

    ///Gets the overall sensor_t data including the type, range and
    //    resulution
    //     @param  sensor Pointer to Adafruit_Sensor sensor_t object that will be
    //    filled with sensor type data
    fn get_sensor(&self) -> SensorInfo {
        SensorInfo {
            name: "TSL2591",
            version: 1,
            sensor_id: self.sensor_id,
            sensor_type: SensorType::Light,
            min_delay: 0,
            max_value: 88000.0,
            min_value: 0.0,
            resolution: 0.001,
        }
    }
}
