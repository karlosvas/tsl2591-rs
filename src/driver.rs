use crate::error::Error;
use crate::registers::{Gain, IntegrationTime, Register};
use crate::registers::{
    TSL2591_COMMAND_BIT, TSL2591_ENABLE_AEN, TSL2591_ENABLE_AIEN, TSL2591_ENABLE_NPIEN,
    TSL2591_ENABLE_POWEROFF, TSL2591_ENABLE_POWERON,
};
use embedded_hal::i2c::I2c;

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

    ///////////////////////////////////////////////////

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

    pub fn begin(&mut self) -> Result<(), Error<I2C::Error>> {
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
}
