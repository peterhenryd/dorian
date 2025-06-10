use dorian_ast::backend::Backend;
use dorian_ast::module::Module;

mod scope;
mod ty;
mod val;
mod llvm;

pub(crate) struct Llvm {
    context: llvm::Context,
    signed_attribute: llvm::Attribute,
    unsigned_attribute: llvm::Attribute,
}

impl Llvm {
    pub(crate) fn new() -> Self {
        let context = llvm::Context::create();
        let signed_attribute = context.create_string_attribute("signage", "signed");
        let unsigned_attribute = context.create_string_attribute("signage", "unsigned");

        Llvm {
            context,
            signed_attribute,
            unsigned_attribute,
        }
    }

    pub(crate) fn context(&self) -> &llvm::Context {
        &self.context
    }
}

impl Backend for Llvm {
    type CompiledModule<'a> = llvm::Module<'a>;

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

            if let Some(signed) = function.ty.return_type.get_signage() {
                if signed {
                    llvm_function.add_attribute(llvm::AttributeLoc::Return, self.signed_attribute);
                } else {
                    llvm_function.add_attribute(llvm::AttributeLoc::Return, self.unsigned_attribute);
                }
            }

            for (i, param) in function.ty.params.iter().enumerate() {
                let Some(signed) = param.get_signage() else {
                    continue;
                };

                let attribute = if signed {
                    self.signed_attribute
                } else {
                    self.unsigned_attribute
                };

                llvm_function.add_attribute(llvm::AttributeLoc::Param(i as u32), attribute);
            }

            self.create_scope(&llvm_module, llvm_function)
                .compile_body(function)
        }

        llvm_module
    }
}
