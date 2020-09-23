/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # MAIR_EL1 - Memory Attribute Indirection Register EL1
//!
//! Provides the memory attribute encodings corresponding to the possible AttrIndx values in a
//! Long-descriptor format translation table entry for stage 1 translations at EL1.
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R/W      | R/W    | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @mair_el1<u64> {
        MAIR0 OFFSET(0) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ],
        MAIR1 OFFSET(8) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ],
        MAIR2 OFFSET(16) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ],
        MAIR3 OFFSET(24) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ],
        MAIR4 OFFSET(32) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ],
        MAIR5 OFFSET(40) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ],
        MAIR6 OFFSET(48) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ],
        MAIR7 OFFSET(56) BITS(8) [
            /// Device Memory nGnRnE
            NGNRNE = 0x00,
            /// Device Memory nGnRE
            NGNRE = 0x04,
            /// Device Memory GRE
            GRE = 0x0C,
            /// Normal Memory Outer and Inner non-cacheable
            NC = 0x44,
            /// Normal Memory Outer and Inner Write Back non transient
            NORM = 0xFF,
            /// Device Memory nGRE
            NGRE = 0x08,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTIWT = 0x33,
            /// Normal Memory Outer and Inner Write Through transient
            NOWTNTIWTNT = 0xBB,
            /// Normal Memory Outer and Inner Write Back transient
            NOWBTIWBT = 0x55
        ]
    }
}
