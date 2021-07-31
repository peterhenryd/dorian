use crate::llvm::sys::LLVMTypeKind;
pub(crate) use crate::llvm::types::Type as LlvmType;
use crate::llvm::types::TypeKind;
use crate::types::int::IntType;
use llvm_sys::core::LLVMGetTypeKind;

pub mod data;
pub mod float;
pub mod fun;
pub mod int;
pub mod ptr;

pub trait Type {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self
    where
        Self: Sized;

    fn get_llvm_type(&self) -> LlvmType;

    fn get_kind(&self) -> TypeKind;

    fn as_int_type(&self) -> Option<IntType> {
        if let TypeKind::Int = self.get_kind() {
            Some(unsafe { IntType::from_llvm_type_unchecked(self.get_llvm_type()) })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct Raw(LlvmType, TypeKind);

fn as_type_kind(llvm_type: &LlvmType) -> TypeKind {
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

impl Type for Raw {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self
    where
        Self: Sized,
    {
        Raw(llvm_type, as_type_kind(&llvm_type))
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        self.1
    }
}
