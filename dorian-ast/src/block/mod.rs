use crate::block::builder::BlockBuilder;
use stmt::Stmt;

pub mod builder;
pub mod stmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Block<'s> {
    pub stmts: Vec<Stmt<'s>>,
}

impl<'s> Block<'s> {
    pub fn new() -> Self {
        Block { stmts: Vec::new() }
    }

    pub fn build() -> BlockBuilder<'s> {
        BlockBuilder::new()
    }
}
