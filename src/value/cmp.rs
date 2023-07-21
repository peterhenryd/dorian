use inkwell::values::AnyValueEnum;
use crate::types::{RawType, Type};
use crate::value::{NonAnyValue, Value};

#[derive(Debug, Copy, Clone)]
pub struct CmpValue<'a>(AnyValueEnum<'a>, RawType<'a>);

impl<'a> CmpValue<'a> {
    pub unsafe fn new_inferred(inkwell_value: AnyValueEnum<'a>) -> Self {
        CmpValue(inkwell_value, RawType::from_inkwell_type_unchecked(inkwell_value.get_type()))
    }
}

impl<'a> Value<'a> for CmpValue<'a> {
    type Type = RawType<'a>;
    type InkwellValue = AnyValueEnum<'a>;

    unsafe fn new_unchecked(value: AnyValueEnum<'a>, raw: Self::Type) -> Self
        where
            Self: Sized,
    {
        CmpValue(value, raw)
    }

    fn get_inkwell_value(&self) -> Self::InkwellValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}

impl<'a> NonAnyValue<'a> for CmpValue<'a>{}