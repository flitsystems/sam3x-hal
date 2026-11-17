# sam3x-hal

Hardware abstraction layer for the Microchip SAM3X family (SAM3X8E,
SAM3X8ERT), a Cortex-M3 microcontroller.

It implements the embedded-hal 1.0 traits, and embedded-io and
embedded-io-async for the serial ports. The async drivers only depend on
the async trait crates, not on an executor: they work with embassy, RTIC or
any other.

## Drivers

- `clocks`: clock tree, PLL and master clock, flash wait states
- `watchdog`: start, feed, disable
- `gpio`: pins with their mode in the type, interrupts, input filters
- `usart`: UART and USART0-3, blocking and async
- `pdc`: transfers through the Peripheral DMA Controller

For the Arduino Due, [`arduino-due`](https://crates.io/crates/arduino-due)
names the pins and sets up the clocks for the board.

## Features

- `rt`: the PAC's runtime, for a binary.
- `defmt`: `defmt::Format` on the public types.

## Minimum Rust version

1.81

## License

Licensed under either of Apache License, Version 2.0 or MIT license, at your
option.

Examples for each driver are in the
[repository](https://github.com/flitsystems/sam3x-hal).
