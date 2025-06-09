use crate::function::Function;
use crate::global::Global;
use crate::structure::Struct;

pub struct Module {
    pub name: String,
    pub structs: Vec<Struct>,
    pub globals: Vec<Global>,
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Module {
            name: name.into(),
            structs: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn add_struct(&mut self, value: Struct) {
        self.structs.push(value);
    }

    pub fn add_global(&mut self, value: Global) {
        self.globals.push(value);
    }

    pub fn add_function(&mut self, value: Function) {
        self.functions.push(value);
    }
}
