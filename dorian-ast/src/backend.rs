use crate::module::Module;

pub trait Backend {
    type CompiledModule<'a>
    where
        Self: 'a;

    fn compile_module<'a>(&'a self, module: &Module) -> Self::CompiledModule<'a>;
}
