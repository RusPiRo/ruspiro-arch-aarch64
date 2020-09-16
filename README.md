# RusPiRo Aarch64 API

This crate provides access to ARM Aarch64 system registers as well as specific assembly instructions. The system registers are organized based on the highest exception level they are available.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-arch-aarch64.svg?branch=release)](https://travis-ci.org/RusPiRo/ruspiro-arch-aarch64)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-arch-aarch64.svg)](https://crates.io/crates/ruspiro-arch-aarch64)
[![Documentation](https://docs.rs/ruspiro-arch-aarch64/badge.svg)](https://docs.rs/ruspiro-arch-aarch64)
[![License](https://img.shields.io/crates/l/ruspiro-arch-aarch64.svg)](https://github.com/RusPiRo/ruspiro-arch-aarch64#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-arch-aarch64 = "0.1.0"
```

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.