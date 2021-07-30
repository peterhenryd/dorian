use crate::llvm::sys::LLVMType;
use std::ptr::NonNull;
use crate::llvm::value::Value;
use crate::llvm::sys::core::{LLVMConstInt, LLVMConstIntOfStringAndSize};
use crate::llvm::to_c_string;

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

impl Type {
    pub unsafe fn const_int(&self, int: u64, sign_extend: bool) -> Value {
        Value::from_raw(
            NonNull::new_unchecked(
                LLVMConstInt(
                    self.0.as_ptr(),
                    int,
                    sign_extend as i32,
                )
            )
        )
    }

    pub unsafe fn const_int_from_str(&self, str: &str, radix: u8) -> Value {
        Value::from_raw(
            NonNull::new_unchecked(
                LLVMConstIntOfStringAndSize(
                    self.0.as_ptr(),
                    to_c_string(Some(str)).as_ptr(),
                    (str.len() + 1) as u32,
                    radix
                )
            )
        )
    }
}