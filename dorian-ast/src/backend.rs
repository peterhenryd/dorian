use crate::module::Module;

pub trait Backend {
    type CompiledModule<'ctx>
    where
        Self: 'ctx;

    fn compile_module<'ctx>(&'ctx self, ast_module: &Module) -> Self::CompiledModule<'ctx>;
}
