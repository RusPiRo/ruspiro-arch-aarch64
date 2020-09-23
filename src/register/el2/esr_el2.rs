/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # ESR_EL2 - Exception Syndrom Register EL2
//!
//! Holds syndrome information for an exception taken to EL2
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | -        | -      | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @esr_el2<u32> {
        /// Syndrome information
        ISS OFFSET(0) BITS(24),
        /// Instruction length for sync exceptions
        IL  OFFSET(25) [
            _16Bit = 0,
            _32Bit = 1
        ],
        /// Exception Class
        EC OFFSET(26) BITS(6)
    }
}
