# tsl2591-rs

Platform-agnostic Rust driver for the **TSL2591** high dynamic range digital light sensor, built on [`embedded-hal`](https://docs.rs/embedded-hal/latest/embedded_hal/).

This project is a Rust port inspired by the Adafruit TSL2591 library.
[Adafruit_TSL2591_Library](https://github.com/adafruit/Adafruit_TSL2591_Library)

## Features

- Public sensor configuration types and default I2C address (`Gain`, `IntegrationTime`, `Persist`, `TSL2591_ADDR`)
- Sensor configuration enums:
  - Integration time
  - Gain
  - Interrupt persistence
- Driver type generic over I2C and delay providers compatible with `embedded-hal` 1.0
- Error type for invalid device ID, sensor overflow, and I2C communication errors

## Project Status

This crate is in an **early stage**.

- Core internals are implemented (register access, lux calculation, data acquisition flow).
- The high-level reading/configuration API is still mostly internal (`fn` methods).
- Public API expansion and hardening are planned before a stable `1.0` release.

If you are evaluating this crate for production, consider it **work in progress** for now.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tsl2591-rs = "0.1.1"
```

## Examples

The `examples/` directory contains usage examples demonstrating various features of the driver. These examples are designed to run on a Raspberry Pi and can be cross-compiled from your development machine.

A convenience script is provided at the root of the project: `run_pi.sh`. It cross-compiles the selected example for `aarch64-unknown-linux-gnu` (Raspberry Pi 64-bit), copies it to the target device via SSH, and executes it remotely. See the script for required environment variables (`PI_USER`, `PI_HOST`, `PI_PATH`).

## Roadmap

- Expose high-level public API:
  - `begin`/device probe
  - gain and integration-time setters/getters
  - raw channel reads
  - lux/event read
- Improve error propagation for I2C operations
- Add integration tests with hardware and/or mocks
- Add complete usage examples

## Development

```bash
cargo check
cargo test
```

## License

BSD-3-Clause. See [LICENSE](LICENSE).

## Acknowledgements

- Inspired by the original Adafruit TSL2591 Arduino library
- Thanks to Adafruit for open-source hardware and software contributions
