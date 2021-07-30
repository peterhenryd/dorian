use crate::llvm::sys::LLVMValue;
use std::ptr::NonNull;
use crate::llvm::types::Type;
use crate::llvm::sys::core::LLVMTypeOf;

#[derive(Copy, Clone)]
pub struct Value(NonNull<LLVMValue>);

impl Value {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMValue>) -> Value {
        Value(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMValue> {
        self.0
    }

    pub fn get_type(&self) -> Type {
        Type::from_raw(
            unsafe {
                NonNull::new_unchecked(
                    LLVMTypeOf(self.0.as_ptr())
                )
            }
        )
    }
}