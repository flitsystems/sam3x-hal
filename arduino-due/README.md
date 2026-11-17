# arduino-due

Board support for the Arduino Due, built on
[`sam3x-hal`](https://crates.io/crates/sam3x-hal).

- The pins under the names printed on the board: `d13`, `a0`, `dac0`,
  `sda`, `can_tx`...
- `init()`: disables the watchdog and sets up the board's clock tree, the
  12 MHz crystal and an 84 MHz core.
- `console()`: the UART on the programming port, which the board's 16U2
  bridges to USB.
- The board's memory layout, `memory.x`, passed to the linker by the build
  script.

## Firmware setup

```toml
[dependencies]
arduino-due = { version = "0.1", features = ["rt"] }
cortex-m-rt = "0.7.5"
```

## Flashing

`flash.sh`, in this crate, is a cargo runner that writes a firmware through
the programming port with bossac. It needs `objcopy` and `bossac`, and takes
the port from `DUE_PORT` (default `/dev/ttyACM0`). Copy it into your project
and set it as the runner in `.cargo/config.toml`.

## Features

- `rt`: the PAC's runtime, for a binary.
- `defmt`: `defmt::Format` on the public types.

## License

Licensed under either of Apache License, Version 2.0 or MIT license, at your
option.

Examples are in the [repository](https://github.com/flitsystems/sam3x-hal).
