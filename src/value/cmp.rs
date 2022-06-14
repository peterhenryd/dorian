use crate::types::{Raw, Type};
use crate::value::{LlvmValue, NonAnyValue, Value};

#[derive(Debug, Copy, Clone)]
pub struct CmpValue(LlvmValue, Raw);

impl CmpValue {
    pub unsafe fn new_inferred(value: LlvmValue) -> Self {
        CmpValue(value, Raw::from_llvm_type_unchecked(value.get_type()))
    }
}

impl Value for CmpValue {
    type Type = Raw;

    unsafe fn new_unchecked(value: LlvmValue, raw: Self::Type) -> Self
        where
            Self: Sized,
    {
        CmpValue(value, raw)
    }

    fn get_llvm_value(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}

impl NonAnyValue for CmpValue {}