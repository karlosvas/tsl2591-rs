use crate::error::Error;
use crate::registers::{
    Gain, IntegrationTime, Register, TSL2591_FULLSPECTRUM, TSL2591_INFRARED, TSL2591_LUX_DF,
    TSL2591_VISIBLE,
};
use crate::registers::{
    TSL2591_COMMAND_BIT, TSL2591_ENABLE_AEN, TSL2591_ENABLE_AIEN, TSL2591_ENABLE_NPIEN,
    TSL2591_ENABLE_POWEROFF, TSL2591_ENABLE_POWERON,
};
use embedded_hal::i2c::I2c;
use std::thread;
use std::time::Duration;

struct AdafruitTSL2591<I2C> {
    i2c: I2C,
    sensor_id: i32,
    integration: IntegrationTime,
    gain: Gain,
    addr: u8,
    initialized: bool,
}

impl<I2C: I2c> AdafruitTSL2591<I2C> {
    pub fn new(i2c: I2C, integration: IntegrationTime, gain: Gain, addr: u8) -> Self {
        AdafruitTSL2591 {
            i2c,
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
            TSL2591_COMMAND_BIT | Register::Control as u8,
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
            TSL2591_COMMAND_BIT | Register::Control as u8,
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

    fn enable(&mut self) {
        self.write8(
            TSL2591_COMMAND_BIT | Register::Enable as u8,
            Some(
                TSL2591_ENABLE_POWERON
                    | TSL2591_ENABLE_AEN
                    | TSL2591_ENABLE_AIEN
                    | TSL2591_ENABLE_NPIEN,
            ),
        );
    }

    fn disable(&mut self) {
        self.write8(
            TSL2591_COMMAND_BIT | Register::Enable as u8,
            Some(TSL2591_ENABLE_POWEROFF),
        );
    }

    fn begin(&mut self) -> Result<(), Error<I2C::Error>> {
        let id: u8 = self.read8(TSL2591_COMMAND_BIT | Register::DeviceID as u8);

        if id != 0x50 {
            return Err(Error::InvalidDevice(id));
        }

        self.initialized = true;

        let integration = self.integration;
        self.set_timing(integration);

        let gain: Gain = self.gain;
        self.set_gain(gain);

        self.disable();

        Ok(())
    }

    fn get_full_luminosity(&mut self) -> u32 {
        self.enable();

        // Wait x ms for ADC to complete
        let cycles: u8 = self.integration as u8 + 1;
        for _ in 0..cycles {
            thread::sleep(Duration::from_millis(120));
        }

        // Empaqueted 32-bit result (lower 16 bits are full spectrum, upper 16 bits are infrared)
        let mut full_spectrum: u32;
        let infrared: u16;
        full_spectrum = self.read16(TSL2591_COMMAND_BIT | Register::Chan0Low as u8) as u32;
        infrared = self.read16(TSL2591_COMMAND_BIT | Register::Chan1Low as u8);

        full_spectrum |= (infrared as u32) << 16;

        self.disable();

        full_spectrum
    }

    fn get_luminosity(&mut self, channel: u8) -> u16 {
        let full_luminosity: u32 = self.get_full_luminosity();

        if channel == TSL2591_FULLSPECTRUM {
            // Reads two byte value from channel 0 (full spectrum)
            (full_luminosity & 0xFFFF) as u16
        } else if channel == TSL2591_INFRARED {
            // Reads two byte value from channel 1 (infrared)
            (full_luminosity >> 16) as u16
        } else if channel == TSL2591_VISIBLE {
            // Reads all and subtracts out just the visible!
            ((full_luminosity & 0xFFFF) - (full_luminosity >> 16)) as u16
        } else {
            // unknown channel!
            0
        }
    }

    fn calculate_lux(&mut self, ch0: u16, ch1: u16) -> Result<f32, Error<I2C::Error>> {
        if (ch0 == 0xFFFF) || (ch1 == 0xFFFF) {
            return Err(Error::Overflow);
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
}
