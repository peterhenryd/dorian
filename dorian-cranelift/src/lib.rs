use dorian_ast::backend::Backend;
use dorian_ast::module::Module;

pub struct Cranelift {}

impl Cranelift {
    pub fn new() -> Self {
        Self {}
    }
}

impl Backend for Cranelift {
    type CompiledModule<'a> = ();

    fn compile_module<'a>(&'a self, _: &Module) -> Self::CompiledModule<'a> {
        todo!("Cranelift backend not implemented yet")
    }
}
