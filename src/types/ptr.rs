use crate::dorian::Dorian;
use crate::llvm::types::TypeKind;
use crate::llvm::AddressSpace;
use crate::types::{LlvmType, Type, CreateType};
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct PtrType<T: Type>(LlvmType, PhantomData<T>);

impl<T: Type> Type for PtrType<T> {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self {
        PtrType(llvm_type, PhantomData::default())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Ptr]
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Ptr
    }
}

impl<T: Type> PtrType<T> {
    pub fn fetch_pointing_type(&self) -> T {
        unsafe {
            T::from_llvm_type_unchecked(
                self.get_llvm_type().get_pointing_type()
            )
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PtrData<T: Type>(T, AddressSpace);

impl<T: Type> PtrData<T> {
    pub fn of(r#type: T, address_space: AddressSpace) -> Self {
        PtrData(r#type, address_space)
    }
}

impl<T: Type + Copy + Clone> CreateType for PtrData<T> {
    type Type = PtrType<T>;

    #[inline(always)]
    fn create(self, _: &Dorian) -> PtrType<T> {
        unsafe { PtrType::from_llvm_type_unchecked(self.0.get_llvm_type().to_ptr_type(self.1)) }
    }
}
