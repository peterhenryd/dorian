pub(crate) use crate::llvm::types::Type as LlvmType;
use crate::llvm::types::TypeKind;
use crate::types::int::IntType;
use crate::types::ptr::PtrType;
use crate::dorian::Dorian;
use std::fmt::Debug;

pub mod void;
pub mod float;
pub mod fun;
pub mod int;
pub mod ptr;
pub mod structure;
pub mod array;

pub trait Type: Debug {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self
    where
        Self: Sized;

    fn valid_kinds() -> Vec<TypeKind>
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

    fn as_ptr_type<T: Type>(&self) -> Option<PtrType<T>> where Self: Sized {
        if let TypeKind::Ptr = self.get_kind() {
            let ptr = unsafe {
                self.get_llvm_type().get_pointing_type()
            };

            if T::valid_kinds().contains(&ptr.get_kind()) {
                return Some(unsafe {
                    PtrType::from_llvm_type_unchecked(
                        self.get_llvm_type(),
                    )
                });
            }
        }

        None
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Raw(LlvmType, TypeKind);

impl Type for Raw {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self
    where
        Self: Sized,
    {
        Raw(llvm_type, TypeKind::of_llvm_type(&llvm_type))
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![
            TypeKind::Void,
            TypeKind::F16,
            TypeKind::F32,
            TypeKind::F64,
            TypeKind::X86F80,
            TypeKind::BF16 ,
            TypeKind::F128,
            TypeKind::PpcF128,
            TypeKind::Label,
            TypeKind::Int,
            TypeKind::Fun,
            TypeKind::Struct,
            TypeKind::Array,
            TypeKind::Ptr,
            TypeKind::Vector,
            TypeKind::Metadata,
            TypeKind::X86Mmx,
            TypeKind::Token,
            TypeKind::ScalableVector,
            TypeKind::X86Amx,
        ]
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        self.1
    }
}

pub trait CreateType: Clone {
    type Type: Type;

    fn create(self, dorian: &Dorian) -> Self::Type;
}