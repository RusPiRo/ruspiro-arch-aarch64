# Changelog

## :banana: v0.1.4

This is mainly a maintenance version migrating the build pipeline to GitHub Actions.

## :melon: v0.1.3

- ### :bulb: Features

  - add new register CSSER_EL1

- ### :wrench: Maintenance

  - provide field value definitions for register CLIDR_EL1
  - fix register size CCSIDR_EL1 to 64Bit

- ### :detective: Fixes
  
  - fix travis-ci config for improved publishing
  - fix travis-ci config as the wrong url for retrieving last version was used

## :strawberry: v0.1.2

The `ruspiro-register` crate was refactored to only contained shared structures and macros eble to be re-used by other crates like this one implementing the register functions. So this version utilizes this crate as dependency.

- ### :wrench: Maintenance

  - Maintained the proper dependency and adjusted the tpe and macros usages
  - Adjusted the file headers to reflect copyright as of 2020 and the correct author

- ### :book: Documentation

  - Fixed minor documentation flaws

## :peach: v0.1.1

This release enhances the documentation in the README.md to appear at [crates.io](https://crates.io) to help users to understand how to use this crate.

- ### :wrench: Maintenance

  - add unit tests for RegisterFieldValue operators `|`, `&` and `==`
  - remove unused macro definitions
  - fix `clippy` task in `Makefile.toml`
  - fix doctests - they will not be executed in a cross compilation fashion, so need to run as host arch. All doc tests need a `no_run` flag to only compile but never execute the code
  - adjust travis doc test task

- ### :book: Documentation

  - Updated README.md to reflect how to use this crate given some examples
  - Added the CONTRIBUTING.md file to document how contribution is handled

## :apple: v0.1.0

This is the initial version of the crate. It is based on a refactoring of the existing `ruspiro-register` crate covering only the functionality and API relevant for the ARM Aarch64 processor architecture. The system register access is grouped into modules based on the lowest exception the register is available. For example a register defined in the module `el2` cannot be accessed from *EL1* or *EL0* but from *EL3* as well. The only register that is not placed in any `el*` module is the one to read the current exception level `currenEL`.

- ### :bulb: Features
  
  - #### EL0 Register

    - CTR_EL0

  - #### EL1 Register

    - CCSIDR_EL1
    - CLIDR_EL1
    - CPACR_EL1
    - ESR_EL1
    - MAIR_EL1
    - MPIDR_EL1
    - SCTLR_EL1
    - TCR_EL1
    - TTBR0_EL1
    - TTBR1_EL1
    - VBAR_EL1

  - #### EL2 Register

    - ACTLR_EL2
    - ESR_EL2
    - HCR_EL2
    - MAIR_EL2
    - SCTLR_EL2
    - TCR_EL2
    - TTBR0_EL2
    - VBAR_EL2

  - #### EL3 Register

    - ACTLR_EL3
    - ESR_EL3

- ### :detective: Fixes
  
  None.

- ### :wrench: Maintenance

  Setup a convinient Travis-CI pipeline to support the verification and publishing of the crate to [crates.io](https://crates.io).

- ### :book: Documentation
  
  Provide an initial documentation of the registers based on the official ARM documentation.
  