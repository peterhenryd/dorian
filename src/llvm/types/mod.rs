use crate::llvm::sys::LLVMType;
use std::ptr::NonNull;

#[derive(Copy, Clone)]
pub struct Type(NonNull<LLVMType>);

impl Type {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMType>) -> Type {
        Type(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMType> {
        self.0
    }
}
