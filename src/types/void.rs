use crate::types::{LlvmType, CreateType, Type};
use crate::dorian::Dorian;
use crate::llvm::types::TypeKind;

/// Represents the void type (type with no possible values).
#[derive(Debug, Copy, Clone)]
pub struct VoidType(LlvmType);

impl Type for VoidType {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self where Self: Sized {
        VoidType(llvm_type)
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Void]
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Void
    }
}

/// Builder for void type.
#[derive(Debug, Copy, Clone)]
pub struct VoidData;

impl CreateType for VoidData {
    type Type = VoidType;

    fn create(self, dorian: &Dorian) -> Self::Type {
        unsafe {
            VoidType::from_llvm_type_unchecked(
                dorian.get_context().get_void_type()
            )
        }
    }
}