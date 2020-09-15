/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Instructions
//!
//! Functions to emit specific assembly instructions
//!

/// assembly NOP instruction
#[inline]
#[allow(dead_code)]
pub fn nop() {
    unsafe { llvm_asm!("nop"::::"volatile") };
}

/// assembly instruction WFE 
#[inline]
#[allow(dead_code)]
pub fn wfe() {
    unsafe { llvm_asm!("wfe") };
}

/// assembly instruction SEV 
#[inline]
#[allow(dead_code)]
pub fn sev() {
    unsafe { llvm_asm!("sev") };
}

/// assembly instruction ISB 
#[inline]
#[allow(dead_code)]
pub fn isb() {
    unsafe { llvm_asm!("isb sy") };
}

/// assembly instruction DSB 
#[inline]
#[allow(dead_code)]
pub fn dsb() {
    unsafe { llvm_asm!("dsb sy") };
}

/// assembly instruction DSB 
#[inline]
#[allow(dead_code)]
pub fn dmb() {
    unsafe { llvm_asm!("dmb sy") };
}
