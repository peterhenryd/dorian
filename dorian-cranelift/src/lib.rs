extern crate dorian_ast as ast;
mod ty;
mod cl;
mod scope;

use ast::backend::Backend;
use ast::function::Function;
use ast::global::Global;
use ast::module::Module;
use ast::structure::Struct;
use target_lexicon::Triple;

pub struct Cranelift {
    module: cl::Module,
    triple: Triple,
    context: cl::FunctionBuilderContext,
    function: Option<cl::Function>,
    namespace: u32,
}

impl Cranelift {
    pub fn new() -> Self {
        Self {
            module: cl::Module::default(),
            triple: Triple::host(),
            context: cl::FunctionBuilderContext::new(),
            function: None,
            namespace: 0,
        }
    }

    fn compile_struct<'ctx>(&'ctx self, _: &Struct) {
        todo!("Structs are not implemented in the Cranelift backend yet");
    }

    fn compile_global<'ctx>(&'ctx self, _: &Global) {
        todo!("Globals are not implemented in the Cranelift backend yet");
    }

    fn compile_function<'ctx>(&'ctx mut self, ast_function: &Function) {
        let signature = self.compile_signature(&ast_function.signature);
        let (func_id, _) = self.module.declare_function(&ast_function.name, cl::Linkage::Export, &signature).unwrap();
        
        let mut scope = self.create_scope(func_id, signature);
        scope.compile_body(&ast_function.body);
        scope.finish();
    }
}

impl Backend for Cranelift {
    type CompiledModule<'ctx> = ();

    fn compile_module<'ctx>(&'ctx mut self, ast_module: &Module) -> Self::CompiledModule<'ctx> {
        for ast_struct in &ast_module.structs {
            self.compile_struct(ast_struct);
        }

        for ast_global in &ast_module.globals {
            self.compile_global(ast_global);
        }

        for ast_function in &ast_module.functions {
            self.compile_function(ast_function);
        }
        
        self.namespace += 1;
    }
}
