/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # VBAR_EL1 - Vector Base Address Register EL1
//!
//! Holds the exception base address for any exception that is taken to EL1
//!
//! ## Usage Constraints
//! EL0 | EL1 (NS) | EL1(S) | EL2 | EL3(NS) | EL3(S)
//! ----|----------|--------|-----|---------|-------
//!  -  | R/W      | R/W    | R/W | R/W     | R/W
//!

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @vbar_el2<u64> {
        /// Base address of the exception vectors for exceptions taken in this exception level
        BADDR OFFSET(11) BITS(53)
    }
}
