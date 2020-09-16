/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Aarch64 - System Register

use core::cmp::PartialEq;
use core::ops::{BitAnd, BitOr, Not, Shl, Shr};

pub mod currentel;
pub mod el0;
pub mod el1;
pub mod el2;
pub mod el3;

mod macros;

/// This trait is used to describe the register size/length as type specifier. The trait is only implemented for the
/// internal types **u8**, **u16**, **u32** and **u64** to ensure safe register access sizes with compile time checking
pub trait RegisterType:
    Copy
    + Clone
    + PartialEq
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + Not<Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
{
}

// Internal macro to ease the assignment of the custom trait to supported register sizes
#[doc(hidden)]
macro_rules! registertype_impl {
    // invoke the macro for a given type t as often as types are provided when invoking the macro
    ($( $t:ty ),*) => ($(
        impl RegisterType for $t { }
    )*)
}

// implement the type trait for specific unsigned types to enable only those register types/sizes
registertype_impl![u8, u16, u32, u64];

/// Definition of a field contained inside of a register. Each field is defined by a mask and the bit shift value
/// when constructing the field definition the stored mask is already shifted by the shift value
#[derive(Copy, Clone, Debug)]
pub struct RegisterField<T: RegisterType> {
    mask: T,
    shift: T,
}

/// Definition of a specific fieldvalue of a regiser. This structure allows to combine field values with bit operators
/// like ``|`` and ``&`` to build the final value that should be written to a register.
#[derive(Copy, Clone, Debug)]
pub struct RegisterFieldValue<T: RegisterType> {
    /// register field definition
    field: RegisterField<T>,
    /// register field value
    value: T,
}

// Internal helper macro to implement:
// - ``RegisterField``struct for all relevant basic types
// - ``FieldValue`` struct for all relevant basic types
// - the operators for ``FieldValue``struct for all relevant basic types
#[doc(hidden)]
macro_rules! registerfield_impl {
    ($($t:ty),*) => ($(
        impl RegisterField<$t> {
            /// Create a new register field definition with the mask and the shift offset for this
            /// mask. The offset is the bit offset this field begins.
            #[inline]
            #[allow(dead_code)]
            pub const fn new(mask: $t, shift: $t) -> RegisterField<$t> {
                Self {
                    mask,
                    shift,
                }
            }

            /// retrieve the current mask of the field shifted to its correct position
            #[inline]
            #[allow(dead_code)]
            pub fn mask(&self) -> $t {
                self.mask.checked_shl(self.shift as u32).unwrap_or(0)
            }

            /// retrieve the current shift of the field
            #[allow(dead_code)]
            #[inline]
            pub fn shift(&self) -> $t {
                self.shift
            }
        }

        impl RegisterFieldValue<$t> {
            /// Create a new fieldvalue based on the field definition and the value given
            #[inline]
            #[allow(dead_code)]
            pub const fn new(field: RegisterField<$t>, value: $t) -> Self {
                RegisterFieldValue {
                    field,
                    value: value & field.mask
                }
            }

            /// Retrieve the register field value
            #[inline]
            #[allow(dead_code)]
            pub fn value(&self) -> $t {
                self.value
            }

            /// Retrieve the register field raw value, means the value is returned in it's position
            /// as it appears in the register when read with the field mask applied but not
            /// shifted
            #[inline]
            #[allow(dead_code)]
            pub fn raw_value(&self) -> $t {
                self.value.checked_shl(self.field.shift as u32).unwrap_or(0)
            }

            /// Retrieve the field mask used with this register field. The mask is shifted to it's
            /// corresponding field position
            #[inline]
            #[allow(dead_code)]
            pub fn mask(&self) -> $t {
                self.field.mask()
            }
        }

        impl PartialEq for RegisterFieldValue<$t> {
            fn eq(&self, other: &Self) -> bool {
                self.value() == other.value()
            }
        }

        impl BitOr for RegisterFieldValue<$t> {
            type Output = RegisterFieldValue<$t>;

            #[inline]
            #[allow(dead_code)]
            fn bitor(self, rhs: RegisterFieldValue<$t>) -> Self {
                let field = RegisterField::<$t>::new( self.field.mask() | rhs.field.mask(), 0);
                RegisterFieldValue {
                    field,
                    value: (self.raw_value() | rhs.raw_value()),
                }
            }
        }

        impl BitAnd for RegisterFieldValue<$t> {
            type Output = RegisterFieldValue<$t>;
            #[inline]
            #[allow(dead_code)]
            fn bitand(self, rhs: RegisterFieldValue<$t>) -> Self {
                let field = RegisterField::<$t>::new( self.field.mask() & rhs.field.mask(), 0);
                RegisterFieldValue {
                    field,
                    value: (self.raw_value() & rhs.raw_value()),
                }
            }
        }
    )*);
}

registerfield_impl![u8, u16, u32, u64];


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register_fieldvalue_eq() {
        let field1 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(3, 0), 7
        );
        let field2 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(3, 0), 7
        );

        assert_eq!(field1, field2);
    }

    #[test]
    fn test_register_fieldvalue_or() {
        let field1 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(0b11, 0), 3
        );
        let field2 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(0b1, 2), 1
        );

        let expected = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(0b111, 0), 7
        );

        assert_eq!(expected, field1 | field2);
    }

    #[test]
    fn test_register_fieldvalue_and() {
        let field1 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(0b11, 0), 3
        );
        let field2 = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(0b11, 1), 1
        );

        let expected = RegisterFieldValue::<u32>::new(
            RegisterField::<u32>::new(0b010, 0), 2
        );

        assert_eq!(expected, field1 & field2);
    }
}