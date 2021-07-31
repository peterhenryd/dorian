use crate::llvm::sys::core::{LLVMConstInt, LLVMConstIntOfStringAndSize, LLVMGetTypeKind, LLVMPointerType, LLVMGetElementType};
use crate::llvm::sys::LLVMType;
use crate::llvm::value::Value;
use crate::llvm::{to_c_string, AddressSpace};
use std::mem::transmute;
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

    pub fn to_ptr_type(&self, address_space: AddressSpace) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMPointerType(self.0.as_ptr(), transmute(address_space)))
        })
    }

    pub unsafe fn get_pointing_type(&self) -> Type {
        Type::from_raw(
            NonNull::new_unchecked(
                LLVMGetElementType(self.0.as_ptr())
            )
        )
    }
}

impl Type {
    pub fn get_kind(&self) -> TypeKind {
        unsafe { transmute(LLVMGetTypeKind(self.0.as_ptr())) }
    }

    pub unsafe fn const_int(&self, int: u64, sign_extend: bool) -> Value {
        Value::from_raw(NonNull::new_unchecked(LLVMConstInt(
            self.0.as_ptr(),
            int,
            sign_extend as i32,
        )))
    }

    pub unsafe fn const_int_from_str(&self, str: &str, radix: u8) -> Value {
        Value::from_raw(NonNull::new_unchecked(LLVMConstIntOfStringAndSize(
            self.0.as_ptr(),
            to_c_string(Some(str)).as_ptr(),
            (str.len() + 1) as u32,
            radix,
        )))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum TypeKind {
    Void = 0,
    F16 = 1,
    F32 = 2,
    F64 = 3,
    X86F80 = 4,
    BF16 = 18,
    F128 = 5,
    PpcF128 = 6,
    Label = 7,
    Int = 8,
    Fun = 9,
    Struct = 10,
    Array = 11,
    Ptr = 12,
    Vector = 13,
    Metadata = 14,
    X86Mmx = 15,
    Token = 16,
    ScalableVector = 17,
    X86Amx = 19,
}
