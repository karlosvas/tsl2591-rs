pub enum Error<E> {
    // Error with I2C bus — wrappea el error del trait embedded-hal
    I2c(E),
    // The device ID doesn't match the expected value
    InvalidDevice(u8),
    // Saturated sensor reading — the value is too high to be represented
    Overflow,
}
