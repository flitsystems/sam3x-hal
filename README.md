# sam3x-hal

Rust support for the Microchip SAM3X family (SAM3X8E, SAM3X8ERT), Cortex-M3.

## Build

The target (`thumbv7m-none-eabi`) and the toolchain are pinned by `.cargo/config.toml` and `rust-toolchain.toml`. `nix develop` provides rustup and git, nothing else is needed.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

The vendored SVD, `atsam3x8e-pac/svd/ATSAM3X8E.svd`, is Atmel's and keeps the license in its header.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
