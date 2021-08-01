use crate::types::{Raw, Type};
use crate::value::{LlvmValue, Value};

#[derive(Debug, Copy, Clone)]
pub struct AnyValue(LlvmValue, Raw);

impl AnyValue {
    pub unsafe fn new_inferred(value: LlvmValue) -> Self {
        AnyValue(value, Raw::from_llvm_type_unchecked(value.get_type()))
    }
}

impl Value for AnyValue {
    type Type = Raw;

    unsafe fn new_unchecked(value: LlvmValue, raw: Self::Type) -> Self
    where
        Self: Sized,
    {
        AnyValue(value, raw)
    }

    fn get_llvm_value(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}
