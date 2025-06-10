use crate::module::Module;

pub trait Backend {
    type CompiledModule<'ctx>
    where
        Self: 'ctx;

    fn compile_module<'ctx>(&'ctx self, module: &Module) -> Self::CompiledModule<'ctx>;
}
