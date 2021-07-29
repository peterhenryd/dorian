use crate::llvm::sys::LLVMValue;
use std::ptr::NonNull;

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
}