use crate::dorian::Dorian;
use crate::llvm::types::TypeKind;
use crate::llvm::AddressSpace;
use crate::types::data::TypeData;
use crate::types::{LlvmType, Type};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct PtrType<T: Type>(LlvmType, PhantomData<T>);

impl<T: Type + Copy + Clone> Type for PtrType<T> {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self {
        PtrType(llvm_type, PhantomData::default())
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Ptr
    }
}

#[derive(Copy, Clone)]
pub struct PtrData<T: Type>(T, AddressSpace);

impl<T: Type> PtrData<T> {
    pub fn of(r#type: T, address_space: AddressSpace) -> Self {
        PtrData(r#type, address_space)
    }
}

impl<T: Type + Copy + Clone> TypeData for PtrData<T> {
    type Type = PtrType<T>;

    #[inline(always)]
    fn create(self, _: &Dorian) -> PtrType<T> {
        unsafe { PtrType::from_llvm_type_unchecked(self.0.get_llvm_type().to_ptr_type(self.1)) }
    }
}
