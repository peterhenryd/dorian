use crate::block::Block;
use crate::stmt::{IfElse, IfStmt, Stmt};
use crate::val::Value;

pub struct BlockBuilder {
    stmts: Vec<Stmt>,
}

impl BlockBuilder {
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }

    pub fn if_then(
        &mut self,
        condition: Value,
        then: impl FnOnce(&mut BlockBuilder),
    ) -> IfStmtBuilder {
        let mut then_block = Block::build();
        then(&mut then_block);

        IfStmtBuilder {
            parent: Parent::Block(self),
            stmt: Some(IfStmt {
                condition: condition.into(),
                then_block: then_block.finish(),
                if_else: None,
            }),
        }
    }

    pub fn ret(&mut self, value: Value) {
        self.stmts.push(Stmt::Return(value.into()));
    }

    pub fn finish(self) -> Block {
        Block { stmts: self.stmts }
    }
}

enum Parent<'a, 'p> {
    Block(&'p mut BlockBuilder),
    IfStmt(&'p mut IfStmtBuilder<'p, 'a>),
}

impl Parent<'_, '_> {
    fn insert(&mut self, stmt: IfStmt) {
        match self {
            Parent::Block(builder) => {
                builder.stmts.push(Stmt::If(stmt));
            }
            Parent::IfStmt(builder) => {
                builder.stmt.as_mut().unwrap().if_else = Some(IfElse::If(Box::new(stmt)))
            }
        }
    }
}

pub struct IfStmtBuilder<'a, 'p> {
    parent: Parent<'a, 'p>,
    stmt: Option<IfStmt>,
}

impl<'a, 'p> IfStmtBuilder<'a, 'p> {
    pub fn else_if<'b>(
        &'a mut self,
        condition: impl Into<Value>,
        then: impl FnOnce(&mut BlockBuilder),
    ) -> IfStmtBuilder<'b, 'a> {
        let mut then_block = Block::build();
        then(&mut then_block);

        IfStmtBuilder {
            parent: Parent::IfStmt(self),
            stmt: Some(IfStmt {
                condition: condition.into(),
                then_block: then_block.finish(),
                if_else: None,
            }),
        }
    }

    pub fn or_else(mut self, else_block: impl FnOnce(&mut BlockBuilder)) {
        let mut block = BlockBuilder::new();
        else_block(&mut block);

        self.stmt.as_mut().unwrap().if_else = Some(IfElse::Else(block.finish()));
    }
}

impl Drop for IfStmtBuilder<'_, '_> {
    fn drop(&mut self) {
        if let Some(stmt) = self.stmt.take() {
            self.parent.insert(stmt);
        }
    }
}
