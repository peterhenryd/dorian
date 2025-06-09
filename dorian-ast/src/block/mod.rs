use crate::block::builder::BlockBuilder;
use crate::stmt::Stmt;

pub mod builder;

#[derive(Debug)]
pub struct Block {
    /// **Note**: While you can do safely, modifying this field directly is not recommended as it requires you to keep
    /// track of contextual information about the block's scope and invocation context. Invalid modifications will lead
    /// to panic upon compilation. It is recommended you use [Block::build] to construct blocks.
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new() -> Self {
        Block { stmts: Vec::new() }
    }

    pub fn build() -> BlockBuilder {
        BlockBuilder::new()
    }
}
