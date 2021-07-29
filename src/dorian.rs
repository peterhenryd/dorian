use crate::llvm::context::Context;
use crate::module::Module;

pub struct Dorian(Context);

impl Dorian {
    #[inline(always)]
    pub fn from_context(context: Context) -> Dorian {
        Dorian(context)
    }

    #[inline(always)]
    pub fn get_context(&self) -> &Context {
        &self.0
    }

    #[inline(always)]
    pub fn new() -> Dorian {
        Dorian::from_context(Context::new())
    }

    pub fn create_module(&self, name: &str) -> Module {
        Module::from_inner(
            self.0.create_module(name)
        )
    }
}