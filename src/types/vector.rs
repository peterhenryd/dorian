use crate::types::{LlvmType, CreateType, Type};
use crate::llvm::types::TypeKind;
use crate::dorian::Dorian;
use std::marker::PhantomData;
use crate::llvm::context::Context;

/// Represents a vector type.
#[derive(Debug, Copy, Clone)]
pub struct VectorType<T: Type>(LlvmType, PhantomData<T>);

impl<T: Type> Type for VectorType<T> {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self where Self: Sized {
        VectorType(llvm_type, PhantomData::default())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Vector]
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Vector
    }
}

/// Builder for vector type.
#[derive(Copy, Clone)]
pub struct VectorData<T: Type + Copy + Clone>(T, u32);

impl<T: Type + Copy + Clone> VectorData<T> {
    pub fn new(r#type: T, size: u32) -> VectorData<T> {
        VectorData(r#type, size)
    }
}

impl<T: Type + Copy + Clone> CreateType for VectorData<T> {
    type Type = VectorType<T>;

    fn create(self, _: &Dorian) -> Self::Type {
        unsafe {
            VectorType::from_llvm_type_unchecked(
                Context::get_vector_type(self.0.get_llvm_type(), self.1)
            )
        }
    }
}