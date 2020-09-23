/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # CCSELR_EL1 - Cache Size Selection Register
//!
//! This is a read-only register
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R/W      | R/W    | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @csselr_el1<u64> {
        /// Instruction not Data Bit
        InD OFFSET(0) [
            /// work on data or unified cache
            D_OR_UNIFIED = 0b0,
            /// work on instruction cache
            INSTRUCTION = 0b1
        ],
        /// The current cache level
        LEVEL OFFSET(1) BITS(3) [
            L1 = 0b000,
            L2 = 0b001,
            L3 = 0b010
        ],
        /// Allocation Tag not Data bit
        TnD OFFSET(4) [
            /// work on data, instruction or unified cache
            D_I_OR_UNIFIED = 0b0,
            /// work on separate allocation tag cache
            ALLOCATION_TAG = 0b1
        ]
    }
}
