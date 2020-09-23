# RusPiRo Aarch64 API

This crate provides access to ARM Aarch64 system registers as well as specific assembly instructions. The system registers are organized based on the highest exception level they are available.

[![Travis-CI Status](https://api.travis-ci.com/RusPiRo/ruspiro-arch-aarch64.svg?branch=release)](https://travis-ci.com/RusPiRo/ruspiro-arch-aarch64)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-arch-aarch64.svg)](https://crates.io/crates/ruspiro-arch-aarch64)
[![Documentation](https://docs.rs/ruspiro-arch-aarch64/badge.svg)](https://docs.rs/ruspiro-arch-aarch64)
[![License](https://img.shields.io/crates/l/ruspiro-arch-aarch64.svg)](https://github.com/RusPiRo/ruspiro-arch-aarch64#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-arch-aarch64 = "||VERSION||"
```

With the dependency maintained in the *Cargo.toml* file the different defined system registers can be accessed as shown in the following code snipped.

For each available system register a mdule with the registers name is available providing functions to read/write the contents of the register. Those read/write accesses can be either use raw values (u32, u64 - based on register size) or predifined register fields. If a register field is used value contained in the field is properly shifted and masked when written to or read from a register.

```rust
use ruspiro_arch_aarch64::register::*;

fn some_function() {
    // read the current exeption level - do this by accessing the predefined
    // register field
    let current_el = currentel::read(currentel::EL::Field);

    if current_el == currentel::EL::EL2 {
        // if a register definition provides predefined constants for specific
        // register fields those can be used for instance to write to the register
        mair_el2::write(
            mair_el2::MAIR0::NGNRNE
                | mair_el2::MAIR1::NGNRE
                | mair_el2::MAIR2::GRE
                | mair_el2::MAIR3::NC
                | mair_el2::MAIR4::NORM,
        );
    }
}
```

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
