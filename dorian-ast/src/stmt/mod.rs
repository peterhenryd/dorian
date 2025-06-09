use crate::block::Block;
use crate::val::Value;

#[derive(Debug)]
pub enum Stmt {
    If(IfStmt),
    Return(Value),
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Value,
    pub then_block: Block,
    pub if_else: Option<IfElse>,
}

#[derive(Debug)]
pub enum IfElse {
    If(Box<IfStmt>),
    Else(Block),
}
