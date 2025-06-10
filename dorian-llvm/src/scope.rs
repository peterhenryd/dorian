use crate::val::LlvmValue;
use crate::Llvm;
use dorian_ast::block::Block;
use dorian_ast::function::Function;
use dorian_ast::stmt::{IfElse, IfStmt, ReturnStmt, Stmt, WhileStmt};
use dorian_ast::val::Var;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::FunctionValue;

impl Llvm {
    pub(crate) fn create_scope<'ctx, 'a>(
        &'ctx self,
        llvm_module: &'a Module<'ctx>,
        llvm_function: FunctionValue<'ctx>,
    ) -> Scope<'ctx, 'a> {
        Scope {
            llvm: self,
            llvm_module,
            llvm_function,
            builder: self.context.create_builder(),
            block: self.context.append_basic_block(llvm_function, "entry"),
            levels: vec![Level::new()],
            depth: 0,
        }
    }
}

pub struct Scope<'ctx, 'a> {
    pub llvm: &'ctx Llvm,
    pub llvm_module: &'a Module<'ctx>,
    pub llvm_function: FunctionValue<'ctx>,
    pub builder: Builder<'ctx>,
    block: BasicBlock<'ctx>,
    levels: Vec<Level<'ctx>>,
    depth: usize,
}

impl<'ctx> Scope<'ctx, '_> {
    pub fn get_var(&self, var: &Var) -> Option<LlvmValue<'ctx>> {
        self.levels
            .get(var.scope_index)?
            .values
            .get(var.item_index)
            .copied()
    }

    pub fn insert_value(&mut self, value: LlvmValue<'ctx>) {
        self.levels[self.depth].values.push(value);
    }

    pub fn compile_body(&mut self, Function { body, .. }: &Function) {
        self.builder.position_at_end(self.block);

        for stmt in &body.stmts {
            self.compile_stmt(stmt);
        }
    }

    fn push_level(&mut self) {
        self.levels.push(Level::new());
        self.depth += 1;
    }

    pub fn pop_level(&mut self) {
        self.levels.pop();
        self.depth -= 1;
    }

    fn compile_block(&mut self, block: &Block) -> bool {
        self.push_level();
        for stmt in &block.stmts {
            if self.compile_stmt(stmt) {
                return true;
            }
        }
        self.pop_level();
        false
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> bool {
        match stmt {
            Stmt::If(x) => self.compile_if_stmt(x),
            Stmt::Return(x) => self.compile_return_stmt(x),
            Stmt::While(x) => self.compile_while_stmt(x),
        }

        matches!(stmt, Stmt::Return(_))
    }

    fn compile_if_stmt(&mut self, stmt: &IfStmt) {
        let condition = self.llvm.compile_value(&stmt.condition, self);
        let then_block = self.append_block();
        let else_block = self.append_block();
        self.builder
            .build_conditional_branch(condition.value.into_int_value(), then_block, else_block)
            .unwrap();

        self.builder.position_at_end(then_block);
        if !self.compile_block(&stmt.then_block) {
            let exit_block = self.append_block();
            self.builder.build_unconditional_branch(exit_block).unwrap();
            self.builder.position_at_end(exit_block);
        }

        if let Some(if_else) = &stmt.if_else {
            self.builder.position_at_end(else_block);
            if !self.compile_if_else(if_else) {
                let exit_block = self.append_block();
                self.builder.build_unconditional_branch(exit_block).unwrap();
                self.builder.position_at_end(exit_block);
            }
        }
    }

    fn compile_if_else(&mut self, if_else: &IfElse) -> bool {
        match if_else {
            IfElse::If(x) => {
                self.compile_if_stmt(x);
                false
            }
            IfElse::Else(x) => self.compile_block(x),
        }
    }

    fn compile_return_stmt(&mut self, stmt: &ReturnStmt) {
        let return_value = stmt
            .value
            .as_ref()
            .map(|x| self.llvm.compile_value(x, self).value);
        match return_value {
            Some(x) => self.builder.build_return(Some(&x)).unwrap(),
            None => self.builder.build_return(None).unwrap(),
        };
    }

    fn append_block(&mut self) -> BasicBlock<'ctx> {
        self.llvm.context.append_basic_block(self.llvm_function, "")
    }

    fn compile_while_stmt(&self, _: &WhileStmt) {
        todo!("While statement compilation is not implemented yet");
    }
}

pub struct Level<'ctx> {
    values: Vec<LlvmValue<'ctx>>,
}

impl<'ctx> Level<'ctx> {
    pub fn new() -> Self {
        Level { values: Vec::new() }
    }
}
