use std::collections::HashMap;
use crate::val::LlvmValue;
use crate::Llvm;
use dorian_ast::block::Block;
use dorian_ast::function::Function;
use dorian_ast::stmt::{AssignStmt, BindStmt, IfElse, IfStmt, ReturnStmt, Stmt, WhileStmt};
use dorian_ast::val::Var;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValue, FunctionValue};

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
    levels: Vec<Level<'ctx>>,
    depth: usize,
}

impl<'ctx> Scope<'ctx, '_> {
    pub fn get_var(&self, var: &Var, load: bool) -> Option<LlvmValue<'ctx>> {
        // Iterative in reverse to prefer the most recent scope and allow for shadowing
        let mut value = None;
        for level in self.levels.iter().rev() {
            if let Some(x) = level.values.get(var.name.as_ref()) {
                value = Some(*x);
                break;
            }
        }

        let value = value?;

        if load {
            let ptr = value.llvm_value.value.into_pointer_value();
            let loaded_value = self.builder.build_load(value.base_type, ptr, &var.name).unwrap();
            Some(LlvmValue {
                value: loaded_value,
                signage: value.llvm_value.signage,
            })
        } else {
            Some(value.llvm_value)
        }
    }

    pub fn compile_body(&mut self, Function { body, .. }: &Function) {
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
        /*
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
                return;
            }
        }

        let exit_block = self.append_block();
        self.builder.build_unconditional_branch(exit_block).unwrap();
        self.builder.position_at_end(else_block);

         */

        let condition = self.llvm.compile_value(&stmt.condition, self);

        // 1. Create all necessary blocks upfront for clarity.
        let then_block = self.append_block();
        let else_block = self.append_block();
        let merge_block = self.append_block();

        // 2. Create the initial conditional branch.
        self.builder
            .build_conditional_branch(condition.value.into_int_value(), then_block, else_block)
            .unwrap();

        // 3. Compile the 'then' block.
        self.builder.position_at_end(then_block);
        // compile_block returns true if it terminates (e.g., with a 'ret' instruction)
        let then_terminates = self.compile_block(&stmt.then_block);
        if !then_terminates {
            // If the 'then' block doesn't return, it must branch to the merge point.
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }

        // 4. Compile the 'else' block.
        self.builder.position_at_end(else_block);
        let else_terminates = if let Some(if_else) = &stmt.if_else {
            // We have an 'else' or 'else if' block. Compile it.
            self.compile_if_else(if_else)
        } else {
            // No 'else' block exists, so this path does not terminate.
            false
        };

        if !else_terminates {
            // If the 'else' branch doesn't terminate (or doesn't exist),
            // it must also branch to the merge point.
            self.builder.build_unconditional_branch(merge_block).unwrap();
        }

        // 5. Position the builder for subsequent code.
        if then_terminates && else_terminates {
            // If both branches return, the merge block is unreachable.
            // We can leave the builder in a dangling state or create a new unreachable block.
            // For simplicity, we just don't position the builder at the merge block.
            // Subsequent code is effectively dead.
            self.builder.build_unconditional_branch(merge_block).unwrap();
        } else {
            // If at least one branch can continue, position the builder at the merge point.
            self.builder.position_at_end(merge_block);
        }

        // 6. The entire 'if' statement terminates ONLY if both of its branches terminate.
        then_terminates && else_terminates
    }

    fn compile_if_else(&mut self, if_else: &IfElse) -> bool {
        match if_else {
            IfElse::If(x) => self.compile_if_stmt(x),
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

    fn compile_while_stmt(&mut self, stmt: &WhileStmt) {
        let condition = self.llvm.compile_value(&stmt.condition, self);

        let exit_block = self.append_block();
        let loop_block = self.append_block();
        self.builder.build_conditional_branch(condition.value.into_int_value(), loop_block, exit_block).unwrap();

        self.builder.position_at_end(loop_block);
        self.compile_block(&stmt.loop_block);
        let condition = self.llvm.compile_value(&stmt.condition, self);
        self.builder.build_conditional_branch(condition.value.into_int_value(), loop_block, exit_block).unwrap();

        self.builder.position_at_end(exit_block);
    }

    fn compile_bind_stmt(&mut self, stmt: &BindStmt) {
        let value = self.llvm.compile_value(&stmt.value, self);
        let base_type = value.value.get_type();
        // TODO: there are more efficient ways to handle variables and avoid stack allocation by increasing context awareness
        let pointer_value = self.builder.build_alloca(base_type, "").unwrap();
        self.builder.build_store(pointer_value, value.value).unwrap();

        let llvm_value = LlvmValue {
            value: pointer_value.as_basic_value_enum(),
            signage: value.signage,
        };

        self.levels[self.depth].values.insert(stmt.name.to_string(), StoredValue { base_type, llvm_value });
    }

    fn compile_assign_stmt(&mut self, stmt: &AssignStmt) {
        let var = self.get_var(&stmt.var, false).unwrap();
        let new_value = self.llvm.compile_value(&stmt.value, self);
        self.builder.build_store(var.value.into_pointer_value(), new_value.value).unwrap();
    }
}

pub struct Level<'ctx> {
    values: HashMap<String, StoredValue<'ctx>>,
}

impl<'ctx> Level<'ctx> {
    pub fn new() -> Self {
        Level { values: HashMap::new() }
    }
}

#[derive(Copy, Clone)]
pub struct StoredValue<'ctx> {
    base_type: BasicTypeEnum<'ctx>,
    llvm_value: LlvmValue<'ctx>,
}
