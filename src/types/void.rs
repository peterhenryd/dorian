use std::marker::PhantomData;
use inkwell::types::{AnyTypeEnum, VoidType as InkwellVoidType};
use crate::types::{CreateType, Type, TypeKind};
use crate::dorian::Dorian;

/// Represents the void type (type with no possible values).
#[derive(Debug, Copy, Clone)]
pub struct VoidType<'a>(InkwellVoidType<'a>);

impl<'a> Type<'a> for VoidType<'a> {
    type InkwellType = InkwellVoidType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum) -> Self where Self: Sized {
        VoidType(inkwell_type.into_void_type())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Void]
    }

    fn get_inkwell_type(&self) -> Self::InkwellType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Void
    }
}

/// Builder for void type.
#[derive(Debug, Copy, Clone)]
pub struct VoidData<'a>(PhantomData<&'a ()>);

impl<'a> VoidData<'a> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }
}

impl<'a> CreateType for VoidData<'a> {
    type Type = VoidType<'a>;

    fn create(self, dorian: &Dorian) -> Self::Type<'_> {
        unsafe {
            VoidType::from_inkwell_type_unchecked(
                dorian.get_context().get_void_type()
            )
        }
    }
}