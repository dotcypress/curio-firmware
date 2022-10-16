# curio-firmware

Firmware for [Curio](https://github.com/dotcypress/curio)
🚧 Work in progress

## Build instructions

1. Install rustup by following the instructions at https://rustup.rs
2. Install Cortex-M0, M0+, and M1 (ARMv6-M architecture) target: `rustup target add thumbv6m-none-eabi`
3. Install LLVM tools: `rustup component add llvm-tools-preview`
4. Install cargo-binutils: `cargo install cargo-binutils` (Note: on some Linux distros (e.g. Ubuntu) you may need to install the packages build-essential, gcc-arm-none-eabi, libssl-dev and pkg-config prior to installing cargo-binutils.)
5. Clone this repo: `git clone git@github.com:dotcypress/curio-firmware.git`
6. Build firmware: `cargo build --release`
7. Flash microcontroller: `cargo run --release`

## Credits

* [`1-Bit Icons`](https://vectorpixelstar.itch.io) by VectorPixelStar
* [`Edition 12`](https://www.dafont.com/edition-12.font) font by Designer's High
* [`BM Army`](https://www.dafont.com/bm-army.font) font by BitmapMania
* [`Nokia Cellphone FC`](https://www.dafont.com/nokia-cellphone.font) font by Zeh Fernando

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
