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
use crate::value::ptr::PtrValue;

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

    fn as_ptr_value<V: Value + Copy + Clone>(&self) -> Option<PtrValue<V>> where V::Type: Copy {
        if let TypeKind::Ptr = self.get_type().get_kind() {
            let ptr = unsafe {
                self.get_type().get_llvm_type().get_pointing_type()
            };

            if V::Type::valid_kinds().contains(&ptr.get_kind()) {
                return Some(unsafe {
                    PtrValue::new_unchecked(
                        self.get_llvm_value(),
                        self.get_type().as_ptr_type().unwrap(),
                    )
                });
            }
        }

        None
    }
}
