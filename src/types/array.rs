use crate::types::{Type, CreateType, TypeKind};
use crate::dorian::Dorian;
use std::marker::PhantomData;
use inkwell::types::{AnyTypeEnum, ArrayType as InkwellArrayType};

/// Represents an array type.
#[derive(Debug, Copy, Clone)]
pub struct ArrayType<'a, T: Type<'a>>(InkwellArrayType<'a>, PhantomData<T>);

impl<'a, T: Type<'a>> Type<'a> for ArrayType<'a, T> {
    type InkwellType = InkwellArrayType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: InkwellArrayType<'a>) -> Self where Self: Sized {
        ArrayType(inkwell_type, PhantomData::default())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![
            TypeKind::Array
        ]
    }

    fn get_inkwell_type(&self) -> InkwellArrayType<'a> {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Array
    }
}

/// Builder for array type.
#[derive(Copy, Clone)]
pub struct ArrayData<'a, T: Type<'a> + Copy + Clone>(T, u32, PhantomData<&'a ()>);

impl<'a, T: Type<'a> + Copy + Clone> ArrayData<'a, T> {
    pub fn new(r#type: T, size: u32) -> ArrayData<'a, T> {
        ArrayData(r#type, size, PhantomData::default())
    }
}

impl<'a, T: Type<'a> + Copy + Clone> CreateType for ArrayData<'a, T> {
    type Type = ArrayType<'a, T>;

    fn create(self, _: &Dorian) -> Self::Type<'_> {
        unsafe {
            ArrayType::from_inkwell_type_unchecked(
                self.0.get_inkwell_type().get_array_type(self.1)
            )
        }
    }
}