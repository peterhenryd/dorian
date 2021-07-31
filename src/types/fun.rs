use crate::dorian::Dorian;
use crate::llvm::types::TypeKind;
use crate::types::data::TypeData;
use crate::types::{LlvmType, Type};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct FunType<R: Type>(LlvmType, PhantomData<R>);

impl<R: Type> Type for FunType<R> {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self
    where
        Self: Sized,
    {
        FunType(llvm_type, PhantomData::default())
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Fun
    }
}

#[derive(Clone)]
pub struct FunData<'a, R: Type + Copy> {
    pub parameters: Vec<&'a dyn Type>,
    pub return_type: &'a R,
    pub is_var_arg: bool,
}

impl<'a, R: Type + Copy> FunData<'a, R> {
    #[inline(always)]
    pub fn new(
        parameters: Vec<&'a dyn Type>,
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

impl<'a, R: Type + Copy> TypeData for FunData<'a, R> {
    type Type = FunType<R>;

    #[inline(always)]
    fn create(self, dorian: &Dorian) -> FunType<R> {
        FunType(
            dorian.get_context().get_fun_type(
                self.parameters.iter().map(|t| t.get_llvm_type()).collect(),
                self.return_type.get_llvm_type(),
                self.is_var_arg,
            ),
            PhantomData::default(),
        )
    }
}
