/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # CPACR_EL1 - Architectural Feature Access Control Register
//!
//! Controls access to trace functionality and access to registers associated with Advanced SIMD
//! and Floating-point execution.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | -        | -      | -   | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @cpacr_el1<u64> {
        // trap floating point instractions in EL0/1
        FPEN OFFSET(20) BITS(2) [
            TRAP_ALL = 0b00,
            TRAP_EL0 = 0b01,
            TRAP_EL0_OR_1 = 0b10,
            NO_TRAP =  0b11
        ],
        // trap trace functions in EL0/1
        TTA OFFSET(28) [
            NO_TRAP = 0b0,
            TRAP =    0b1
        ]
    }
}
