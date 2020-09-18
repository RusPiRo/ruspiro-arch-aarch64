/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache 2.0
 **********************************************************************************************************************/

//! # TCR_EL1 - Translation Control Register EL1
//!
//! Determines which Translation Base Registers defines the base address register for a translation
//! table walk required for stage 1 translation of a memory access from EL0 or EL1 and holds
//! cacheability and shareability information.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//! R/W | R/W      | R/W    | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @ctr_el0<u64> {
        /// Log2 of the number of words in the smallest cache line of all the instruction caches that are controlled by
        /// the PE
        IminLine    OFFSET(0) BITS(4),
        /// Level 1 instruction cache policy. Indicates the indexing and tagging policy for the L1 instruction cache
        L1lp        OFFSET(14) BITS(2) [
            /// VMID aware Physical Index, Physical tag
            VPIPT   = 0b00,
            /// ASID-tagged Virtual Index, Virtual Tag
            AIVIVT  = 0b01,
            /// Virtual Index, Physical Tag
            VIPT    = 0b10,
            /// Physical Index, Physical Tag
            PIPT    = 0b11
        ],
        /// Log2 of the number of words in the smallest cache line of all the data caches and unified caches that are
        /// controlled by the PE.
        DminLine    OFFSET(16) BITS(4),
        /// Exclusives reservation granule. Log2 of the number of words of the maximum size of the reservation granule
        /// that has been implemented for the Load-Exclusive and Store-Exclusive instructions.
        ERG         OFFSET(20) BITS(4),
        /// Cache writeback granule. Log2 of the number of words of the maximum size of memory that can be overwritten
        /// as a result of the eviction of a cache entry that has had a memory location in it modified.
        CWG         OFFSET(24) BITS(4),
        /// Data cache clean requirements for instruction to data coherence.
        IDC         OFFSET(28),
        TminLine    OFFSET(32) BITS(6)
    }
}
