/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Aarch64 - System Register

//use core::cmp::PartialEq;
//use core::ops::{BitAnd, BitOr, Not, Shl, Shr};
use ruspiro_register::{RegisterField, RegisterFieldValue};

pub mod currentel;
pub mod el0;
pub mod el1;
pub mod el2;
pub mod el3;

mod macros;
