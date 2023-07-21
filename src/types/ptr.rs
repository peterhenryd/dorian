use crate::dorian::Dorian;
use crate::types::{Type, CreateType, TypeKind};
use std::marker::PhantomData;
use inkwell::AddressSpace;
use inkwell::types::{AnyTypeEnum, PointerType};

/// Represents a pointer type.
#[derive(Debug, Copy, Clone)]
pub struct PtrType<'a, T: Type<'a>>(PointerType<'a>, PhantomData<T>);

impl<'a, T: Type<'a>> Type<'a> for PtrType<'a, T> {
    type InkwellType = PointerType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum<'a>) -> Self {
        PtrType(inkwell_type.into_pointer_type(), PhantomData::default())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Ptr]
    }

    fn get_inkwell_type(&self) -> PointerType<'a> {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Ptr
    }
}

impl<'a, T: Type<'a>> PtrType<'a, T> {
    pub fn fetch_pointing_type(&self) -> T {
        unsafe {
            T::from_inkwell_type_unchecked(
                self.get_inkwell_type().get_pointing_type()
            )
        }
    }
}

/// Builder for pointer type.
#[derive(Debug, Copy, Clone)]
pub struct PtrData<'a, T: Type<'a>>(T, AddressSpace, PhantomData<&'a ()>);

impl<'a, T: Type<'a>> PtrData<'a, T> {
    pub fn of(r#type: T, address_space: AddressSpace) -> Self {
        PtrData(r#type, address_space, PhantomData::default())
    }
}

impl<'a, T: Type<'a> + Copy + Clone> CreateType for PtrData<'a, T> {
    type Type = PtrType<'a, T>;

    #[inline(always)]
    fn create(self, _: &Dorian) -> PtrType<'a, T> {
        unsafe { PtrType::from_inkwell_type_unchecked(self.0.get_inkwell_type().to_ptr_type(self.1)) }
    }
}
