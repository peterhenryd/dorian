use inkwell::types::AnyTypeEnum;
use inkwell::values::AnyValueEnum;
use crate::types::{RawType, Type};
use crate::value::Value;

/// Represents a value that doesn't have a [Type].
#[derive(Debug, Copy, Clone)]
pub struct AnyValue<'a>(AnyValueEnum<'a>, RawType<'a>);

impl<'a> AnyValue<'a> {
    /// Creates a new [AnyValue].
    pub unsafe fn new_inferred(value: AnyValueEnum) -> Self {
        AnyValue(value, RawType::from_inkwell_type_unchecked(value.get_type()))
    }
}

impl<'a> Value<'a> for AnyValue<'a> {
    type Type = RawType<'a>;
    type InkwellValue = AnyValueEnum<'a>;

    unsafe fn new_unchecked(value: AnyValueEnum<'a>,  raw: Self::Type) -> Self
    where
        Self: Sized,
    {
        AnyValue(value, raw)
    }

    fn get_inkwell_value(&self) -> AnyValueEnum<'a> {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}
