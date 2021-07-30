use crate::types::{Type, TypeKind, LlvmType};
use crate::types::data::TypeData;
use crate::dorian::Dorian;

#[derive(Copy, Clone)]
pub struct FnType(LlvmType);

impl<'a> Type<'a> for FnType {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self where Self: Sized {
        FnType(llvm_type)
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Fn
    }
}

#[derive(Clone)]
pub struct FnData<'a> {
    pub parameters: Vec<&'a dyn Type<'a>>,
    pub return_type: &'a dyn Type<'a>,
    pub is_var_arg: bool
}

impl<'a> FnData<'a> {
    #[inline(always)]
    pub fn new(parameters: Vec<&'a dyn Type<'a>>, return_type: &'a dyn Type<'a>, is_var_arg: bool) -> FnData<'a> {
        FnData { parameters, return_type, is_var_arg }
    }
}

impl<'a> TypeData<'a> for FnData<'a> {
    type Type = FnType;

    #[inline(always)]
    fn create(self, dorian: &Dorian) -> FnType {
        FnType(
            dorian.get_context().get_function_type(
                self.parameters.iter()
                    .map(|t| t.get_llvm_type())
                    .collect(),
                self.return_type.get_llvm_type(),
                self.is_var_arg
            )
        )
    }
}