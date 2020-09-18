/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 * 
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! Aarch64 - EL2 Register
//!
//! Each iof the contained modules represents a system register and allows access to it. Please refer to the module 
//! documtation to get details about the contained register.

pub mod actlr_el2;
pub mod esr_el2;
pub mod hcr_el2;
pub mod mair_el2;
pub mod sctlr_el2;
pub mod tcr_el2;
pub mod ttbr0_el2;
pub mod vbar_el2;
