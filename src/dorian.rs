use crate::llvm::context::Context;
use crate::module::Module;

/// This structure encapsulates the [Context] for the sole purpose of creating modules (as well
/// as associating modules together).
///
/// For other uses of [Context], the context itself can be borrowed.
pub struct Dorian(Context);

impl Dorian {
    /// Creates a [Dorian] instance from a pre-existing [Context].
    pub fn from_llvm(context: Context) -> Dorian {
        Dorian(context)
    }

    /// Borrow the internal [Context] for more complicated operations.
    pub fn get_context(&self) -> &Context {
        &self.0
    }

    /// Creates a [Dorian] instance.
    pub fn new() -> Dorian {
        Dorian::from_llvm(Context::new())
    }

    /// Creates a module under a [Dorian] instance.
    pub fn create_module(&self, name: &str) -> Module {
        Module::from_llvm(self, self.0.create_module(name))
    }
}
