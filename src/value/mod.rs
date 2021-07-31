pub mod any;
pub mod constant;
pub mod data;
pub mod float;
pub mod int;
pub mod ptr;

use crate::llvm::types::TypeKind;
pub(crate) use crate::llvm::value::Value as LlvmValue;
use crate::types::Type;
use crate::value::int::IntValue;

pub trait Value {
    type Type: Type;

    unsafe fn new_unchecked(value: crate::llvm::value::Value, _: Self::Type) -> Self
    where
        Self: Sized;

    fn get_llvm_value(&self) -> crate::llvm::value::Value;

    fn get_type(&self) -> &Self::Type;

    fn as_int_value(&self) -> Option<IntValue> {
        if let TypeKind::Int = self.get_type().get_kind() {
            Some(unsafe {
                IntValue::new_unchecked(
                    self.get_llvm_value(),
                    self.get_type().as_int_type().unwrap(),
                )
            })
        } else {
            None
        }
    }
}
