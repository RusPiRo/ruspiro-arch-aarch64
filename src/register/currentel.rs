/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 * 
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # CurrentEL - Current Exception Level
//!
//! Holds the current exception level
//!
//! ```no_run
//! # use ruspiro_arch_aarch64::register::*;
//!
//! # fn main() {
//! // read the current exeption level - do this by accessing the predefined
//! // register field
//! let current_el = currentel::read(currentel::EL::Field);
//!
//! if current_el == currentel::EL::EL2 {
//!     /* do something */
//! }
//! # }
//! ```

use crate::register::*;
use crate::{define_aarch64_register, impl_system_register_rw};

define_aarch64_register! {
    @currentEl<u64> {
        /// The current exception level
        EL OFFSET(2) BITS(2) [
            EL0 = 0b00,
            EL1 = 0b01,
            EL2 = 0b10,
            EL3 = 0b11
        ]
    }
}
