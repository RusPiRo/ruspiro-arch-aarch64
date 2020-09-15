/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Register definition macros
//!
//! The macros are used to simplify the definition of system registers.
//!

/// Helper macro to define the fields a register may contain of.<br>
/// This is typically part of the register definition and will be applied there. It's not intended for use outside
/// of a register definition.
#[doc(hidden)]
#[macro_export]
macro_rules! register_field {
    ($t:ty, $field:ident, $offset:expr) => {
        #[allow(unused_variables, dead_code)]
        #[doc(hidden)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new(1, $offset);
    };
    ($t:ty, $field:ident, $offset:expr, $bits:expr) => {
        #[allow(unused_variables, dead_code)]
        #[doc(hidden)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new((1 << $bits) - 1, $offset);
    };
}

/// Helper macro to implement shared register functions for aarch32/64 system registers
#[doc(hidden)]
#[macro_export]
macro_rules! impl_system_register_rw {
    ($t:ty) => {
        /// Update the contents of a register from the ``RegisterFieldValue`` given. This will
        /// only change the bits the ``RegisterField`` definition specifies.
        #[inline]
        #[allow(dead_code)]
        pub fn write(field_value: RegisterFieldValue<$t>) {
            let raw_value = (get() & !field_value.mask()) | field_value.raw_value();
            set(raw_value);
        }

        /// Read the contents of a specific ``RegisterField``. The returned value is already shifted
        /// to the right to start at bit 0. This means for a field value stored in the register at
        /// bit offset 3, the returned value is already shifted by 3 bits to the right.
        /// For example:
        /// If register raw value is 0b10110, the returned value for a register field specified as
        /// bits\[4:3\] would be 0b01. No further "masking" or "bit-shift" required
        #[inline]
        #[allow(dead_code)]
        pub fn read(field: RegisterField<$t>) -> RegisterFieldValue<$t> {
            let raw_value = get() & field.mask();
            RegisterFieldValue::<$t>::new(field, raw_value >> field.shift())
        }
    };
}

/// Macro to define an Aarch64 system register and its fields
///
/// # Examples
/// ```no_run
/// # #![feature(llvm_asm)]
/// # use ruspiro_platform::*;
///
/// define_aarch64_register!(
///     /// Aarch64 register foo as 32Bit register
///     /// It's always a Read/Write register definition, check the register documentation
///     /// to verify write's are allowed
///     foo<u32> {
///         /// Register field BAR with it's enum fields
///         BAR OFFSET(0) [
///             /// Field value VAL1
///             VAL1 = 0b1,
///             VAL0 = 0b0
///         ],
///         /// Register field BAR with it's own
///         /// enum like field values
///         BAZ OFFSET(1) BITS(2) [
///             VAL1 = 0b01,
///             VAL2 = 0b10,
///             VAL3 = 0b11
///         ]
///     }
/// );
///
/// # fn main() {
///     foo::write(
///         foo::BAR::VAL1 | foo::BAZ::VAL2
///     );
/// # }
///
#[macro_export]
macro_rules! define_aarch64_register {
    (@$(#[doc = $rdoc:expr])*
      $name:ident<$t:ty> {
        $($(#[doc = $fdoc:expr])* $field:ident OFFSET($offset:expr) $(BITS($bits:expr))? $([
            $($(#[doc = $fvdoc:expr])* $enum:ident = $value:expr),*
        ])?),*
      }) => {
        $(
            $(#[doc = $fdoc])*
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            pub mod $field {
                #[allow(unused_imports)]
                use super::*;

                register_field!($t, Field, $offset $(, $bits)?);

                /// Create a ``RegisterFieldValue`` from the current ``RegisterField``
                /// of this ``Register`` from a given value
                #[inline]
                #[allow(dead_code)]
                pub fn with_value(value: $t) -> RegisterFieldValue<$t> {
                    RegisterFieldValue::<$t>::new(Field, value)
                }

                $(
                    $(
                        $(#[doc = $fvdoc])*
                        #[allow(unused_variables, dead_code)]
                        pub const $enum: RegisterFieldValue::<$t> = RegisterFieldValue::<$t>::new(Field, $value);
                    )*
                )*
            }
        )*

        /// Read the raw register contents using the appropriate assembly
        #[inline]
        #[allow(dead_code)]
        pub fn get() -> $t {
            let raw_value: $t;
            unsafe {
                llvm_asm!(concat!("mrs $0,", stringify!($name)):"=r"(raw_value):::"volatile")
            };
            raw_value
        }

        /// Write the raw register contents using the appropriate contents
        #[inline]
        #[allow(dead_code)]
        pub fn set(raw_value: $t) {
            unsafe {
                llvm_asm!(concat!("msr ", stringify!($name) , ", $0 ")::"r"(raw_value)::"volatile")
            }
        }

        impl_system_register_rw!($t);

    };

    ($(#[doc = $rdoc:expr])*
     $name:ident<$t:ty> {
        $($(#[doc = $fdoc:expr])*
        $field:ident OFFSET($offset:expr) $(BITS($bits:expr))? $([
            $($(#[doc = $fvdoc:expr])* $enum:ident = $value:expr),*
        ])?),*
    }) => {
        $(#[doc = $rdoc])*
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        pub mod $name {
            #[allow(unused_imports)]
            use $crate::*;
            $crate::define_aarch64_register!{
                @$(#[doc = $rdoc])?
                $name<$t> {
                    $($(#[doc = $fdoc])* $field OFFSET($offset) $(BITS($bits))? $([
                        $(
                            $(#[doc = $fvdoc])*
                            $enum = $value
                        ),*
                    ])?),*
                }
            }
        }
    };
}
