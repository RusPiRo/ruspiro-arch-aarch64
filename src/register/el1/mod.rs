/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Aarch64 - EL1 Register
//!
//! Each iof the contained modules represents a system register and allows access to it. Please refer to the module 
//! documtation to get details about the contained register.

pub mod ccsidr_el1;
pub mod clidr_el1;
pub mod cpacr_el1;
pub mod esr_el1;
pub mod mair_el1;
pub mod mpidr_el1;
pub mod sctlr_el1;
pub mod tcr_el1;
pub mod ttbr0_el1;
pub mod ttbr1_el1;
pub mod vbar_el1;
