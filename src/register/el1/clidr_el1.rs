/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # CLIDR_EL1 - Cache Level ID Register
//!
//! This is a read-only register
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R        | R      | R   | R       | R
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @clidr_el1<u64> {
        /// Type of cache implemented at L1
        CTYPE1 OFFSET(0) BITS(3) [
            /// No cache at this level
            NO_CACHE = 0b000,
            /// Only instruction cache
            ICACHE_ONLY = 0b001,
            /// Only data cache
            DCACHE_ONLY = 0b010,
            /// Separate data and instruction cache
            SEPARATE_D_I_CACHE = 0b011,
            /// unified data and instruction cache
            UNIFIED_D_I_CACHE = 0b100
        ],
        /// Type of cache implemented at L2
        CTYPE2 OFFSET(3) BITS(3) [
            /// No cache at this level
            NO_CACHE = 0b000,
            /// Only instruction cache
            ICACHE_ONLY = 0b001,
            /// Only data cache
            DCACHE_ONLY = 0b010,
            /// Separate data and instruction cache
            SEPARATE_D_I_CACHE = 0b011,
            /// unified data and instruction cache
            UNIFIED_D_I_CACHE = 0b100
        ],
        /// Type of cache implemented at L3
        CTYPE3 OFFSET(6) BITS(3) [
            /// No cache at this level
            NO_CACHE = 0b000,
            /// Only instruction cache
            ICACHE_ONLY = 0b001,
            /// Only data cache
            DCACHE_ONLY = 0b010,
            /// Separate data and instruction cache
            SEPARATE_D_I_CACHE = 0b011,
            /// unified data and instruction cache
            UNIFIED_D_I_CACHE = 0b100
        ],
        /// Level of Unification inner sharable for cache hierarchy
        LOUIS OFFSET(21) BITS(3),
        /// Level of Coherency for cache hierarchy
        LOC OFFSET(24) BITS(3),
        /// Level of Unification Uniprocessor for cache hierarchy
        LOUU OFFSET(27) BITS(3),
        /// Inner cache boundary
        ICB OFFSET(30) BITS(3)
    }
}
