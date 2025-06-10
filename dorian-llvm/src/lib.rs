extern crate dorian_ast as ast;

use ast::backend::Backend;
use ast::global::Global;
use ast::module::Module;
use ast::structure::Struct;
use crate::scope::Scope;

mod scope;
mod ty;
mod val;
mod llvm;

pub struct Llvm {
    context: llvm::Context,
    signed_attribute: llvm::Attribute,
    unsigned_attribute: llvm::Attribute,
}

impl Llvm {
    pub fn new() -> Self {
        let context = llvm::Context::create();
        let signed_attribute = context.create_string_attribute("signage", "signed");
        let unsigned_attribute = context.create_string_attribute("signage", "unsigned");

        Llvm {
            context,
            signed_attribute,
            unsigned_attribute,
        }
    }
    
    fn compile_struct<'ctx>(&'ctx self, ast_struct: &Struct) {
        let struct_type = self.context.opaque_struct_type(&ast_struct.name);
        let fields = ast_struct
            .fields
            .iter()
            .map(|field_type| self.compile_data_type(field_type))
            .collect::<Vec<_>>();

        struct_type.set_body(&fields, false);
    }
    
    fn compile_global<'ctx>(&'ctx self, ast_global: &Global, module: &llvm::Module<'ctx>) {
        let global_type = self.compile_data_type(&ast_global.ty);
        let global = module.add_global(global_type, None, &ast_global.name);

        if let Some(ast_value) = &ast_global.value {
            if let Some(value) = self.compile_value(ast_value, Scope::Global) {
                global.set_initializer(&value.raw);
            } else {
                panic!("Attempted to initialize global variable '{}' with a value that could not be compiled in the global scope", ast_global.name);
            }
        }

        let _ = global;
    }
    
    fn compile_functions<'ctx>(&'ctx self, ast_module: &Module, module: &llvm::Module<'ctx>) {
        let mut pairs = Vec::with_capacity(ast_module.functions.len());
        for ast_function in &ast_module.functions {
            let function_type = self.compile_function_type(&ast_function.ty);
            let function = module.add_function(&ast_function.name, function_type, None);

            if let Some(signed) = ast_function.ty.return_type.get_signage() {
                if signed {
                    function.add_attribute(llvm::AttributeLoc::Return, self.signed_attribute);
                } else {
                    function.add_attribute(llvm::AttributeLoc::Return, self.unsigned_attribute);
                }
            }

            for (i, param) in ast_function.ty.params.iter().enumerate() {
                let Some(signed) = param.get_signage() else {
                    continue;
                };

                let attribute = if signed {
                    self.signed_attribute
                } else {
                    self.unsigned_attribute
                };

                function.add_attribute(llvm::AttributeLoc::Param(i as u32), attribute);
            }

            pairs.push((ast_function, function));
        }

        for (ast_function, function) in pairs {
            self.create_scope(&module, function)
                .compile_body(ast_function);
        }
    }
}

impl Backend for Llvm {
    type CompiledModule<'a> = llvm::Module<'a>;

    fn compile_module<'a>(&'a self, ast_module: &Module) -> Self::CompiledModule<'a> {
        let module = self.context.create_module(&ast_module.name);

        for ast_struct in &ast_module.structs {
            self.compile_struct(ast_struct);
        }

        for ast_global in &ast_module.globals {
            self.compile_global(ast_global, &module);
        }
        
        self.compile_functions(ast_module, &module);

        module
    }
}
