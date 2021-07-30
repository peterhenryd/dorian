pub mod block;

use crate::function::block::Block;
use crate::value::{Value, AnyValue};
use crate::types::{Raw, Type};

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
            self.0.get_context().create_builder(),
            self.0.append_basic_block("")
        )
    }

    pub fn fetch_params(&self) -> Vec<AnyValue> {
        self.0.get_params()
            .iter()
            .map(|&v| unsafe { AnyValue::new_unchecked(v, Raw::from_llvm_type_unchecked(v.get_type())) })
            .collect()
    }
}