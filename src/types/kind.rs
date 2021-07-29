use crate::types::Type;
use crate::dorian::Dorian;

pub trait TypeData<'a> {
    fn create(&'a self, dorian: &Dorian) -> Type<'a>;

    fn get_kind(&self) -> TypeKind;
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeKind {
    Int,
    Fn,
}

#[derive(Copy, Clone)]
pub struct IntData {
    pub bits: u32
}

impl IntData {
    #[inline(always)]
    pub fn new(bits: u32) -> IntData {
        IntData { bits }
    }
}

impl<'a> TypeData<'a> for IntData {
    #[inline(always)]
    fn create(&'a self, dorian: &Dorian) -> Type<'a> {
        Type(
            dorian.get_context().get_integer_type(self.bits),
            self
        )
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Int
    }
}

#[derive(Clone)]
pub struct FnData<'a> {
    pub parameters: Vec<Type<'a>>, pub return_type: Type<'a>, pub is_var_arg: bool
}

impl<'a> FnData<'a> {
    #[inline(always)]
    pub fn new(parameters: Vec<Type<'a>>, return_type: Type<'a>, is_var_arg: bool) -> FnData<'a> {
        FnData { parameters, return_type, is_var_arg }
    }
}

impl<'a> TypeData<'a> for FnData<'a> {
    #[inline(always)]
    fn create(&'a self, dorian: &Dorian) -> Type<'a> {
        Type(
            dorian.get_context().get_function_type(
                self.parameters.iter()
                    .map(|t| t.get_inner())
                    .collect(),
                self.return_type.get_inner(),
                self.is_var_arg
            ),
            self
        )
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Fn
    }
}