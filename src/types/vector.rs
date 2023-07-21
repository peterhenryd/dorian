use crate::types::{CreateType, Type, TypeKind};
use crate::dorian::Dorian;
use std::marker::PhantomData;
use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, VectorType as InkwellVectorType};

/// Represents a vector type.
#[derive(Debug, Copy, Clone)]
pub struct VectorType<'a, T: Type<'a>>(InkwellVectorType<'a>, PhantomData<T>);

impl<'a, T: Type<'a>> Type<'a> for VectorType<'a, T> {
    type InkwellType = InkwellVectorType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum) -> Self where Self: Sized {
        VectorType(inkwell_type.into_vector_type(), PhantomData::default())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Vector]
    }

    fn get_inkwell_type(&self) -> Self::InkwellType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Vector
    }
}

/// Builder for vector type.
#[derive(Copy, Clone)]
pub struct VectorData<'a, T: Type<'a> + Copy + Clone>(T, u32, PhantomData<&'a ()>);

impl<'a, T: Type<'a> + Copy + Clone> VectorData<'a, T> {
    pub fn new(r#type: T, size: u32) -> VectorData<'a, T> {
        VectorData(r#type, size, PhantomData::default())
    }
}

impl<'a, T: Type<'a> + Copy + Clone> CreateType for VectorData<'a, T> {
    type Type<'b> = VectorType<'a, T>;

    fn create(self, _: &Dorian) -> Self::Type<'a> {
        todo!()/*
        unsafe {
            VectorType::from_inkwell_type_unchecked(
                Context::type(self.0.get_inkwell_type(), self.1)
            )
        }
        */
    }
}