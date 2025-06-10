use std::borrow::Cow;
use crate::block::Block;
use crate::block::stmt::{AssignStmt, BindStmt, IfElse, IfStmt, ReturnStmt, Stmt, WhileStmt};
use crate::val::{Value, Var};

#[derive(Debug, Clone, PartialEq)]
pub struct BlockBuilder<'s> {
    stmts: Vec<Stmt<'s>>,
}

impl<'s> BlockBuilder<'s> {
    pub fn new() -> Self {
        Self { stmts: vec![] }
    }

    pub fn if_then(
        &mut self,
        condition: Value<'s>,
        build: impl FnOnce(&mut BlockBuilder),
    ) -> IfStmtBuilder<'s, '_> {
        let mut then_block = Block::build();
        build(&mut then_block);

        IfStmtBuilder {
            parent: Parent::Block(self),
            stmt: Some(IfStmt {
                condition: condition.into(),
                then_block: then_block.finish(),
                if_else: None,
            }),
        }
    }

    pub fn loop_while(
        &mut self,
        condition: Value<'s>,
        build: impl FnOnce(&mut BlockBuilder),
    ) {
        let mut loop_block = Block::build();
        build(&mut loop_block);

        self.stmts.push(Stmt::While(WhileStmt {
            condition: condition.into(),
            loop_block: loop_block.finish(),
        }));
    }

    pub fn ret(&mut self, value: Value<'s>) {
        self.stmts
            .push(Stmt::Return(ReturnStmt { value: Some(value) }));
    }

    pub fn bind(&mut self, name: impl Into<Cow<'s, str>>, value: Value<'s>) {
        let name = name.into();
        self.stmts.push(Stmt::Bind(BindStmt {
            name: name.clone(),
            value: value.into(),
        }));
    }

    pub fn assign(&mut self, var: Var<'s>, value: Value<'s>) {
        self.stmts.push(Stmt::Assign(AssignStmt { var, value }));
    }

    pub fn finish(self) -> Block<'s> {
        Block { stmts: self.stmts }
    }
}

enum Parent<'s, 'p> {
    Block(&'p mut BlockBuilder<'s>),
    IfStmt(&'p mut IfStmtBuilder<'s, 'p>),
}

impl<'s> Parent<'s, '_> {
    fn insert(&mut self, stmt: IfStmt<'s>) {
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

pub struct IfStmtBuilder<'s, 'p> {
    parent: Parent<'s, 'p>,
    stmt: Option<IfStmt<'s>>,
}

impl<'s, 'p> IfStmtBuilder<'s, 'p> {
    pub fn else_if<'b: 'p>(
        &'b mut self,
        condition: impl Into<Value<'s>>,
        then: impl FnOnce(&mut BlockBuilder),
    ) -> IfStmtBuilder<'s, 'b> {
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
