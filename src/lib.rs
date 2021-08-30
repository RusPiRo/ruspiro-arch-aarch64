/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-arch-aarch64/||VERSION||")]
// we require to run with 'std' in unit tests and doc tests to have an allocator in place
//#![cfg_attr(not(any(test, doctest)), no_std)]
#![no_std]
#![feature(asm)]

//! # RusPiRo Aarch64 specific API
//!
//! This crate provides access to Aarch64 system registers as well as specific usefull aarch64 assembly instructions
//!

pub use ruspiro_register::*;

pub mod instructions;
pub mod register;
