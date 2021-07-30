use crate::llvm::module::Module as LLVMModule;
use crate::function::Function;
use crate::types::{Type, TypeKind};
use crate::types::function::FnType;

pub struct Module<'a>(LLVMModule<'a>);

impl<'a> Module<'a> {
    pub fn from_inner(inner: LLVMModule<'a>) -> Module<'a> {
        Module(inner)
    }

    pub fn get_inner(&self) -> &LLVMModule<'a> {
        &self.0
    }

    pub fn add_fn<'b>(&mut self, name: &str, fn_type: &FnType) -> Function {
        unsafe {
            Function::from_raw(
                self.0.add_function(name, fn_type.get_llvm_type())
            )
        }
    }
}

impl ToString for Module<'_> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}