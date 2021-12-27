/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # Instructions
//!
//! Functions to emit specific assembly instructions
//!
use core::arch::asm;

/// assembly NOP instruction
#[inline]
#[allow(dead_code)]
pub fn nop() {
  unsafe { asm!("nop") };
}

/// assembly instruction WFE
#[inline]
#[allow(dead_code)]
pub fn wfe() {
  unsafe { asm!("wfe") };
}

/// assembly instruction SEV
#[inline]
#[allow(dead_code)]
pub fn sev() {
  unsafe { asm!("sev") };
}

/// assembly instruction ISB
#[inline]
#[allow(dead_code)]
pub fn isb() {
  unsafe { asm!("isb sy") };
}

/// assembly instruction DSB
#[inline]
#[allow(dead_code)]
pub fn dsb() {
  unsafe { asm!("dsb sy") };
}

/// assembly instruction DSB
#[inline]
#[allow(dead_code)]
pub fn dmb() {
  unsafe { asm!("dmb sy") };
}
