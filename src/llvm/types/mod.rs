use crate::llvm::sys::core::{LLVMConstInt, LLVMConstIntOfStringAndSize, LLVMGetTypeKind, LLVMPointerType, LLVMGetElementType, LLVMArrayType};
use crate::llvm::sys::{LLVMType, LLVMTypeKind};
use crate::llvm::value::Value;
use crate::llvm::{to_c_string, AddressSpace};
use std::mem::transmute;
use std::ptr::NonNull;
use crate::types::LlvmType;

#[derive(Debug, Copy, Clone)]
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

    pub fn get_array_type(&self, size: u32) -> Type {
        Type::from_raw(
            unsafe {
                NonNull::new_unchecked(
                    LLVMArrayType(self.0.as_ptr(), size)
                )
            }
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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TypeKind {
    // TODO: Implement
    Void = 0,
    F16 = 1,
    F32 = 2,
    F64 = 3,
    X86F80 = 4,
    BF16 = 18,
    F128 = 5,
    PpcF128 = 6,
    // TODO: Implement
    Label = 7,
    Int = 8,
    Fun = 9,
    Struct = 10,
    // TODO: Implement
    Array = 11,
    Ptr = 12,
    // TODO: Implement
    Vector = 13,
    // TODO: Implement
    Metadata = 14,
    // TODO: Implement
    X86Mmx = 15,
    // TODO: Implement
    Token = 16,
    // TODO: Implement
    ScalableVector = 17,
    // TODO: Implement
    X86Amx = 19,
}

impl TypeKind {
    #[inline(always)]
    pub fn of_llvm_type(llvm_type: &LlvmType) -> TypeKind {
        let kind = unsafe { LLVMGetTypeKind(llvm_type.as_raw().as_ptr()) };
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => TypeKind::Void,
            LLVMTypeKind::LLVMHalfTypeKind => TypeKind::F16,
            LLVMTypeKind::LLVMFloatTypeKind => TypeKind::F32,
            LLVMTypeKind::LLVMDoubleTypeKind => TypeKind::F64,
            LLVMTypeKind::LLVMX86_FP80TypeKind => TypeKind::X86F80,
            LLVMTypeKind::LLVMFP128TypeKind => TypeKind::F128,
            LLVMTypeKind::LLVMPPC_FP128TypeKind => TypeKind::PpcF128,
            LLVMTypeKind::LLVMLabelTypeKind => TypeKind::Label,
            LLVMTypeKind::LLVMIntegerTypeKind => TypeKind::Int,
            LLVMTypeKind::LLVMFunctionTypeKind => TypeKind::Fun,
            LLVMTypeKind::LLVMStructTypeKind => TypeKind::Struct,
            LLVMTypeKind::LLVMArrayTypeKind => TypeKind::Array,
            LLVMTypeKind::LLVMPointerTypeKind => TypeKind::Ptr,
            LLVMTypeKind::LLVMVectorTypeKind => TypeKind::Vector,
            LLVMTypeKind::LLVMMetadataTypeKind => TypeKind::Metadata,
            LLVMTypeKind::LLVMX86_MMXTypeKind => TypeKind::X86Mmx,
            LLVMTypeKind::LLVMTokenTypeKind => TypeKind::Token,
            LLVMTypeKind::LLVMScalableVectorTypeKind => TypeKind::ScalableVector,
            LLVMTypeKind::LLVMBFloatTypeKind => TypeKind::BF16,
            LLVMTypeKind::LLVMX86_AMXTypeKind => TypeKind::X86Amx,
        }
    }
}