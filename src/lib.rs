/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 * 
 * Author: 2ndTaleStudio <43264484+2ndTaleStudio@users.noreply.github.com>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-arch-aarch64/||VERSION||")]
// we require to run with 'std' in unit tests and doc tests to have an allocator in place
#![cfg_attr(not(any(test, doctests)), no_std)]
#![feature(llvm_asm, const_fn)]
#![cfg(any(target_arch = "aarch64", test, doctests))]
//! # RusPiRo Aacrh64 specific API
//! 
//! This crate provides access to Aarch64 system registers as well as specific usefull aarch64 assembly instructions
//! 

pub mod instructions;
pub mod register;