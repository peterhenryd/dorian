use crate::block::Block;
use crate::block::builder::BlockBuilder;
use crate::ty::{ConcreteType, DataType, VoidType};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub ty: FunctionType,
    pub body: Block,
}

impl Function {
    pub fn new(name: impl Into<String>) -> Self {
        Function {
            name: name.into(),
            ty: FunctionType::default(),
            body: Block::new(),
        }
    }

    pub fn with_return_type(mut self, return_type: ConcreteType) -> Self {
        self.ty.return_type = return_type;
        self
    }

    pub fn add_param(mut self, param: DataType) -> Self {
        self.ty = self.ty.with_param(param);
        self
    }

    pub fn with_block(mut self, block: Block) -> Self {
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
pub struct FunctionType {
    pub params: Vec<DataType>,
    pub return_type: ConcreteType,
}

impl FunctionType {
    pub fn new(return_type: ConcreteType) -> Self {
        FunctionType {
            params: Vec::new(),
            return_type: return_type.into(),
        }
    }

    pub fn with_param(mut self, param: DataType) -> Self {
        self.params.push(param);
        self
    }
}

impl Default for FunctionType {
    fn default() -> Self {
        Self {
            params: Vec::new(),
            return_type: ConcreteType::Void(VoidType),
        }
    }
}
