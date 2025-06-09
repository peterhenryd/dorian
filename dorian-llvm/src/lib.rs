use dorian_ast::backend::Backend;
use dorian_ast::module::Module;
use inkwell::context::Context as LlvmContext;
use inkwell::module::Module as LlvmModule;

mod scope;
mod ty;
mod val;

pub struct Llvm {
    context: LlvmContext,
}

impl Llvm {
    pub fn new() -> Self {
        Self {
            context: LlvmContext::create(),
        }
    }

    pub fn context(&self) -> &LlvmContext {
        &self.context
    }
}

impl Backend for Llvm {
    type CompiledModule<'a> = LlvmModule<'a>;

    fn compile_module<'a>(&'a self, module: &Module) -> Self::CompiledModule<'a> {
        let llvm_module = self.context.create_module(&module.name);

        for structure in &module.structs {
            let llvm_struct_type = self.context.opaque_struct_type(&structure.name);
            let fields = structure
                .fields
                .iter()
                .map(|field_type| self.compile_data_type(field_type))
                .collect::<Vec<_>>();

            llvm_struct_type.set_body(&fields, false);
        }

        for global in &module.globals {
            let llvm_global_type = self.compile_data_type(&global.ty);
            let llvm_global = llvm_module.add_global(llvm_global_type, None, &global.name);

            let _ = llvm_global;
        }

        for function in &module.functions {
            let llvm_function_type = self.compile_function_type(&function.ty);
            let llvm_function = llvm_module.add_function(&function.name, llvm_function_type, None);

            self.create_scope(&llvm_module, llvm_function)
                .compile_body(function)
        }

        llvm_module
    }
}
