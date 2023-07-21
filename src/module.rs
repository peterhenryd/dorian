use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::{Module as InkwellModule};
use inkwell::OptimizationLevel;
use inkwell::support::LLVMString;
use crate::dorian::Dorian;
use crate::fun::Fun;
use crate::types::fun::FunType;
use crate::types::Type;

/// Represents a module consisting of functions, structures, and other symbols.
pub struct Module<'a> {
    dorian: &'a Dorian,
    inkwell_module: InkwellModule< 'a >
}

impl<'a> Module<'a> {
    /// Creates a [Module] from an [LlvmModule]
    pub fn from_llvm(dorian: &'a Dorian, inkwell_module: InkwellModule<'a>) -> Module<'a> {
        Module { dorian, inkwell_module }
    }

    /// Borrows the internal [LlvmModule].
    pub fn get_inner(&self) -> &InkwellModule<'a> {
        &self.1
    }

    /// Adds a function to the current module, and returns a [Fun] for adding a body.
    pub fn add_fn<R: Type<'a>>(&mut self, name: &str, fun_type: &FunType<'a, R>) -> Fun<'a, R> {
        unsafe { Fun::new(self.0, self.1.add_fun(name, fun_type.get_inkwell_type())) }
    }


    #[inline(always)]
    pub fn create_jit_execution_engine(
        &self,
        optimization_level: OptimizationLevel,
    ) -> Result<ExecutionEngine<'a>, LLVMString> {
        self.inkwell_module.create_jit_execution_engine(optimization_level)
    }
}

impl ToString for Module<'_> {
    fn to_string(&self) -> String {
        self.1.to_string()
    }
}
