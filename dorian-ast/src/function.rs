use std::borrow::Cow;
use crate::block::Block;
use crate::block::builder::BlockBuilder;
use crate::ty::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Function<'s> {
    pub name: Cow<'s, str>,
    pub signature: Signature,
    pub body: Block<'s>,
}

impl<'s> Function<'s> {
    pub fn new(name: impl Into<Cow<'s, str>>) -> Self {
        Function {
            name: name.into(),
            signature: Signature {
                input: Vec::new(),
                output: Vec::new(),
            },
            body: Block::new(),
        }
    }

    pub fn add_input(mut self, param: Type) -> Self {
        self.signature.input.push(param);
        self
    }

    pub fn add_output(mut self, output: Type) -> Self {
        self.signature.output.push(output);
        self
    }

    pub fn with_block(mut self, block: Block<'s>) -> Self {
        self.body = block;
        self
    }

    pub fn build_block(self, f: impl FnOnce(&mut BlockBuilder)) -> Self {
        let mut builder = BlockBuilder::new();
        f(&mut builder);

        self.with_block(builder.finish())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Signature {
    pub input: Vec<Type>,
    pub output: Vec<Type>,
}