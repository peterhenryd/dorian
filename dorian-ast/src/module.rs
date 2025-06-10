use std::borrow::Cow;
use crate::function::Function;
use crate::global::Global;
use crate::structure::Struct;

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'s> {
    pub name: Cow<'s, str>,
    pub structs: Vec<Struct<'s>>,
    pub globals: Vec<Global<'s>>,
    pub functions: Vec<Function<'s>>,
}

impl<'s> Module<'s> {
    pub fn new(name: impl Into<Cow<'s, str>>) -> Self {
        Module {
            name: name.into(),
            structs: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn add_struct(&mut self, value: Struct<'s>) {
        self.structs.push(value);
    }

    pub fn add_global(&mut self, value: Global<'s>) {
        self.globals.push(value);
    }

    pub fn add_function(&mut self, value: Function<'s>) {
        self.functions.push(value);
    }
}
