pub mod block;

use crate::function::block::Block;

pub struct Function<'a>(crate::llvm::function::Function<'a>);

impl<'a> Function<'a> {
    pub unsafe fn from_raw(inner: crate::llvm::function::Function<'a>) -> Function<'a> {
        Function(inner)
    }

    pub fn get_inner(&self) -> &crate::llvm::function::Function<'a> {
        &self.0
    }

    pub fn add_block(&mut self) -> Block {
        Block::from_llvm(
            self.0.append_basic_block("")
        )
    }
}