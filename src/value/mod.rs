pub mod any;
pub mod constant;
pub mod data;
pub mod float;
pub mod int;
pub mod ptr;
pub mod fun;
pub mod cmp;
pub mod vector;

use inkwell::values::{AnyValue as InkwellValue, AnyValueEnum};
use crate::types::{RawType, Type, TypeKind};
use crate::value::any::AnyValue;
use crate::value::int::IntValue;
use crate::value::ptr::PtrValue;

pub trait NonAnyValue<'a>: Value<'a> {}

/// Represents an instance of a type, also known as a value.
pub trait Value<'a> {
    /// The [Type] of the value.
    type Type: Type<'a>;
    type InkwellValue: InkwellValue<'a>;

    /// Creates a value from an [crate::llvm::value::Value].
    unsafe fn new_unchecked(value: AnyValueEnum<'a>, ty: Self::Type) -> Self
    where
        Self: Sized;

    /// Borrow the internal [crate::llvm::value::Value].
    fn get_inkwell_value(&self) -> Self::InkwellValue;

    /// Borrow the [Type] of the value.
    fn get_type(&self) -> &Self::Type;

    /// Will return a [Some] of an [IntValue] if the value is an underlying integer value.
    fn as_int_value(&self) -> Option<IntValue> {
        if let TypeKind::Int = self.get_type().get_kind() {
            Some(unsafe {
                IntValue::new_unchecked(
                    self.get_inkwell_value(),
                    self.get_type().as_int_type().unwrap(),
                )
            })
        } else {
            None
        }
    }

    /// Will return a [Some] of an [PtrValue] if the value is an underlying pointer value.
    fn as_ptr_value<V: Value<'a> + Copy + Clone>(&self) -> Option<PtrValue<'a, V>> where V::Type: Copy {
        if let TypeKind::Ptr = self.get_type().get_kind() {
            let ptr = unsafe {
                self.get_type().get_inkwell_type().get_pointing_type()
            };

            if V::Type::valid_kinds().contains(&ptr.get_kind()) {
                return Some(unsafe {
                    PtrValue::new_unchecked(
                        self.get_inkwell_value(),
                        self.get_type().as_ptr_type().unwrap(),
                    )
                });
            }
        }

        None
    }

    /// Will return a [Some] of an [PtrValue] if the value is an underlying pointer value.
    fn as_any_value(&self) -> AnyValue {
        unsafe { AnyValue::new_inferred(self.get_inkwell_value()) }
    }
}

impl<'a, V: NonAnyValue<'a>> From<V> for AnyValue<'a> {
    fn from(value: V) -> Self {
        unsafe {
            AnyValue::new_unchecked(
                value.get_inkwell_value(),
                RawType::from_inkwell_type_unchecked(value.get_type().get_inkwell_type())
            )
        }
    }
}