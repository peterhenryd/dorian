use crate::types::{LlvmType, Type, CreateType};
use crate::llvm::types::TypeKind;
use crate::dorian::Dorian;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct ArrayType<T: Type>(LlvmType, PhantomData<T>);

impl<T: Type> Type for ArrayType<T> {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self where Self: Sized {
        ArrayType(llvm_type, PhantomData::default())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![
            TypeKind::Array
        ]
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Array
    }
}

#[derive(Copy, Clone)]
pub struct ArrayData<T: Type + Copy + Clone>(T, u32);

impl<T: Type + Copy + Clone> ArrayData<T> {
    pub fn new(r#type: T, size: u32) -> ArrayData<T> {
        ArrayData(r#type, size)
    }
}

impl<T: Type + Copy + Clone> CreateType for ArrayData<T> {
    type Type = ArrayType<T>;

    fn create(self, _: &Dorian) -> Self::Type {
        unsafe {
            ArrayType::from_llvm_type_unchecked(
                self.0.get_llvm_type().get_array_type(self.1)
            )
        }
    }
}