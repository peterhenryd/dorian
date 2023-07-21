use crate::dorian::Dorian;
use crate::types::{Type, CreateType, TypeKind, RawType};
use std::marker::PhantomData;
use inkwell::types::{AnyType, AnyTypeEnum, FunctionType};

/// Represents a function type.
#[derive(Debug, Copy, Clone)]
pub struct FunType<'a, R: Type<'a>>(FunctionType<'a>, PhantomData<R>);

impl<'a, R: Type<'a>> Type<'a> for FunType<'a, R> {
    type InkwellType = FunctionType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum<'a>) -> Self
    where
        Self: Sized,
    {
        FunType(inkwell_type.into_function_type(), PhantomData::default())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Fun]
    }

    fn get_inkwell_type(&self) -> Self::InkwellType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Fun
    }
}

/// Builder for function type.
#[derive(Debug, Clone)]
pub struct FunData<'a, R: Type<'a>> {
    pub parameters: Vec<RawType<'a>>,
    pub return_type: &'a R,
    pub is_var_arg: bool,
}

impl<'a, R: Type<'a> + Copy> FunData<'a, R> {
    #[inline(always)]
    pub fn new(
        parameters: Vec<RawType<'a>>,
        return_type: &'a R,
        is_var_arg: bool,
    ) -> FunData<'a, R> {
        FunData {
            parameters,
            return_type,
            is_var_arg,
        }
    }
}

impl<'a, R: Type<'a> + Copy> CreateType for FunData<'a, R> {
    type Type = FunType<'a, R>;

    #[inline(always)]
    fn create(self, dorian: &Dorian) -> FunType<'a, R> {
        FunType(
            dorian.get_context().get_fun_type(
                self.parameters.iter().map(|t| t.get_inkwell_type()).collect(),
                self.return_type.get_inkwell_type(),
                self.is_var_arg,
            ),
            PhantomData::default(),
        )
    }
}
