use crate::dorian::Dorian;
use crate::fun::Fun;
use crate::llvm::execution_engine::ExecutionEngine;
use crate::llvm::module::Module as LlvmModule;
use crate::llvm::OptimizationLevel;
use crate::types::fun::FunType;
use crate::types::Type;

pub struct Module<'a>(&'a Dorian, LlvmModule<'a>);

impl<'a> Module<'a> {
    pub fn from_llvm(dorian: &'a Dorian, llvm_module: LlvmModule<'a>) -> Module<'a> {
        Module(dorian, llvm_module)
    }

    pub fn get_inner(&self) -> &LlvmModule<'a> {
        &self.1
    }

    pub fn add_fn<R: Type>(&mut self, name: &str, fun_type: &FunType<R>) -> Fun<'a, R> {
        unsafe { Fun::new(self.0, self.1.add_fun(name, fun_type.get_llvm_type())) }
    }

    pub fn create_execution_engine(
        &self,
        optimization_level: OptimizationLevel,
    ) -> ExecutionEngine {
        ExecutionEngine::new(self, optimization_level)
    }
}

impl ToString for Module<'_> {
    fn to_string(&self) -> String {
        self.1.to_string()
    }
}
