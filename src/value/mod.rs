pub mod data;
pub mod float;
pub mod int;
pub mod constant;

pub(crate) use crate::llvm::value::Value as LlvmValue;
use crate::value::int::IntValue;
use crate::types::{Type, TypeKind, Raw};

pub trait Value<'a> {
    type Type: Type<'a> + ?Sized;

    unsafe fn new_unchecked(value: crate::llvm::value::Value, _: Self::Type) -> Self where Self: Sized;

    fn get_inner(&self) -> crate::llvm::value::Value;

    fn get_type(&self) -> &Self::Type;

    fn as_int_value(&self) -> Option<IntValue> {
        if let TypeKind::Int = self.get_type().get_kind() {
            Some(unsafe {
                IntValue::new_unchecked(
                    self.get_inner(),
                    self.get_type().as_int_type().unwrap()
                )
            })
        } else {
            None
        }
    }
}

impl<'a, T: Type<'a>> dyn Value<'a, Type = T> {
    pub fn as_any(&self) -> AnyValue {
        AnyValue(self.get_inner(), unsafe { Raw::from_llvm_type_unchecked(self.get_type().get_llvm_type()) })
    }
}

#[derive(Copy, Clone)]
pub struct AnyValue(LlvmValue, Raw);

impl AnyValue {
    unsafe fn new_inferred(value: LlvmValue) -> Self {
        AnyValue(value, Raw::from_llvm_type_unchecked(value.get_type()))
    }
}

impl<'a> Value<'a> for AnyValue {
    type Type = Raw;

    unsafe fn new_unchecked(value: LlvmValue, raw: Self::Type) -> Self where Self: Sized {
        AnyValue(value, raw)
    }

    fn get_inner(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}