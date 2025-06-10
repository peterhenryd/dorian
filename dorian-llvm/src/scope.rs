use std::collections::HashMap;
use inkwell::values::BasicValue;
use crate::{llvm, Llvm};
use dorian_ast::block::Block;
use dorian_ast::function::Function;
use dorian_ast::block::stmt::{AssignStmt, BindStmt, IfElse, IfStmt, ReturnStmt, Stmt, WhileStmt};
use dorian_ast::val::Var;

impl Llvm {
    pub(crate) fn create_scope<'ctx, 'a>(
        &'ctx self,
        llvm_module: &'a llvm::Module<'ctx>,
        llvm_function: llvm::Function<'ctx>,
    ) -> Scope<'ctx, 'a> {
        Scope {
            llvm: self,
            module: llvm_module,
            function: llvm_function,
            builder: self.context.create_builder(),
            levels: vec![Level::new()],
            depth: 0,
        }
    }
}

pub(crate) struct Scope<'ctx, 'a> {
    pub(crate) llvm: &'ctx Llvm,
    pub(crate) module: &'a llvm::Module<'ctx>,
    pub(crate) function: llvm::Function<'ctx>,
    pub(crate) builder: llvm::Builder<'ctx>,
    levels: Vec<Level<'ctx>>,
    depth: usize,
}

impl<'ctx> Scope<'ctx, '_> {
    pub(crate) fn get_var(&self, var: &Var, load: bool) -> Option<llvm::Value<'ctx>> {
        // Iterate in reverse to prefer the most recent scope and allow for shadowing
        let mut stored_value = None;
        for level in self.levels.iter().rev() {
            if let Some(x) = level.values.get(var.name.as_ref()) {
                stored_value = Some(*x);
                break;
            }
        }

        let stored_value = stored_value?;
        if load {
            let pointer_value = stored_value.value.raw.into_pointer_value();
            let loaded_value = self.builder.build_load(stored_value.base_type, pointer_value, &var.name).unwrap();
            Some(llvm::Value::new(loaded_value, stored_value.value.signage))
        } else {
            Some(stored_value.value)
        }
    }

    pub(crate) fn compile_body(&mut self, Function { body, .. }: &Function) {
        let block = self.append_block();
        self.builder.position_at_end(block);

        for stmt in &body.stmts {
            self.compile_stmt(stmt);
        }
    }

    fn push_level(&mut self) {
        self.levels.push(Level::new());
        self.depth += 1;
    }

    fn pop_level(&mut self) {
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
            Stmt::Return(x) => {
                self.compile_return_stmt(x);
                true
            },
            Stmt::While(x) => {
                self.compile_while_stmt(x);
                false
            },
            Stmt::Bind(x) => {
                self.compile_bind_stmt(x);
                false
            },
            Stmt::Assign(x) => {
                self.compile_assign_stmt(x);
                false
            },
        }
    }

    fn compile_if_stmt(&mut self, stmt: &IfStmt) -> bool {
        let condition = self.llvm.compile_value(&stmt.condition, self);

        let then_block = self.append_block();
        let else_block = self.append_block();
        let merge_block = self.append_block();
        
        self.builder.build_conditional_branch(condition.raw.into_int_value(), then_block, else_block).unwrap();
        self.builder.position_at_end(then_block);

        let then_terminates = self.compile_block(&stmt.then_block);
        if !then_terminates {
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }

        let else_terminates = if let Some(if_else) = &stmt.if_else {
            self.compile_if_else(if_else)
        } else {
            false
        };

        if !else_terminates {
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }

        if then_terminates && else_terminates {
            self.builder.build_unconditional_branch(merge_block).unwrap();
        } else {
            self.builder.position_at_end(merge_block);
        }

        then_terminates && else_terminates
    }

    fn compile_if_else(&mut self, if_else: &IfElse) -> bool {
        match if_else {
            IfElse::If(x) => self.compile_if_stmt(x),
            IfElse::Else(x) => self.compile_block(x),
        }
    }

    fn compile_return_stmt(&mut self, stmt: &ReturnStmt) {
        let Some(ast_value) = &stmt.value else {
            self.builder.build_return(None).unwrap();
            return;
        };
        
        let value = self.llvm.compile_value(ast_value, self);
        self.builder.build_return(Some(&value.raw)).unwrap();
    }

    fn append_block(&mut self) -> llvm::Block<'ctx> {
        self.llvm.context.append_basic_block(self.function, "")
    }

    fn compile_while_stmt(&mut self, stmt: &WhileStmt) {
        let condition = self.llvm.compile_value(&stmt.condition, self);

        let exit_block = self.append_block();
        let loop_block = self.append_block();
        self.builder.build_conditional_branch(condition.raw.into_int_value(), loop_block, exit_block).unwrap();

        self.builder.position_at_end(loop_block);
        self.compile_block(&stmt.loop_block);
        
        let condition = self.llvm.compile_value(&stmt.condition, self);
        self.builder.build_conditional_branch(condition.raw.into_int_value(), loop_block, exit_block).unwrap();

        self.builder.position_at_end(exit_block);
    }

    fn compile_bind_stmt(&mut self, stmt: &BindStmt) {
        let stored_value = self.llvm.compile_value(&stmt.value, self);
        let stored_type = stored_value.raw.get_type();
        
        // TODO: there are more efficient ways to handle variables and avoid stack allocation by increasing context awareness
        let pointer_value = self.builder.build_alloca(stored_type, "").unwrap();
        self.builder.build_store(pointer_value, stored_value.raw).unwrap();
        
        let raw_value = pointer_value.as_basic_value_enum();
        let value = llvm::Value::new(raw_value, stored_value.signage);

        self.levels[self.depth].values.insert(stmt.name.to_string(), StoredValue { base_type: stored_type, value });
    }

    fn compile_assign_stmt(&mut self, stmt: &AssignStmt) {
        let var = self.get_var(&stmt.var, false).unwrap();
        let new_value = self.llvm.compile_value(&stmt.value, self);
        self.builder.build_store(var.raw.into_pointer_value(), new_value.raw).unwrap();
    }
}

pub(crate) struct Level<'ctx> {
    values: HashMap<String, StoredValue<'ctx>>,
}

impl<'ctx> Level<'ctx> {
    pub(crate) fn new() -> Self {
        Level { values: HashMap::new() }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct StoredValue<'ctx> {
    base_type: llvm::Type<'ctx>,
    value: llvm::Value<'ctx>,
}
