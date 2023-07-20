use crate::dorian::Dorian;
use crate::fun::Fun;
use crate::llvm::execution_engine::ExecutionEngine;
use crate::llvm::module::Module as LlvmModule;
use crate::llvm::OptimizationLevel;
use crate::types::fun::FunType;
use crate::types::Type;

/// Represents a module consisting of functions, structures, and other symbols.
pub struct Module<'a>(&'a Dorian, LlvmModule<'a>);

impl<'a> Module<'a> {
    /// Creates a [Module] from an [LlvmModule]
    pub fn from_llvm(dorian: &'a Dorian, llvm_module: LlvmModule<'a>) -> Module<'a> {
        Module(dorian, llvm_module)
    }

    /// Borrows the internal [LlvmModule].
    pub fn get_inner(&self) -> &LlvmModule<'a> {
        &self.1
    }

    /// Adds a function to the current module, and returns a [Fun] for adding a body.
    pub fn add_fn<R: Type>(&mut self, name: &str, fun_type: &FunType<R>) -> Fun<'a, R> {
        unsafe { Fun::new(self.0, self.1.add_fun(name, fun_type.get_llvm_type())) }
    }


    #[inline(always)]
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
