pub mod block;

use std::marker::PhantomData;
use crate::dorian::Dorian;
use crate::fun::block::Block;
use crate::llvm::fun::Fun as LlvmFun;
use crate::types::{Raw, Type};
use crate::value::any::AnyValue;
use crate::value::Value;

/// This structure represents a function in the context of a module.
pub struct Fun<'a, R: Type>(&'a Dorian, LlvmFun<'a>, PhantomData<R>);

impl<'a, R: Type> Fun<'a, R> {
    /// Creates a function from an [LlvmFun].
    pub unsafe fn new(dorian: &'a Dorian, llvm_fun: LlvmFun<'a>) -> Fun<'a, R> {
        Fun(dorian, llvm_fun, PhantomData::default())
    }

    /// Borrow the internal [LlvmFun].
    pub fn get_inner(&self) -> &LlvmFun<'a> {
        &self.1
    }

    pub fn get_return_type(&self) -> R {
        unsafe { R::from_llvm_type_unchecked(self.1.get_return_type()) }
    }

    /// Add a named block to the function.
    pub fn add_block(&mut self, name: &str) -> Block<'a, R> {
        Block::new(
            self.0,
            self.1.get_context().create_builder(),
            self.1.append_basic_block(name),
        )
    }

    /// Returns the values inputted when the function is called.
    pub fn fetch_params(&self) -> Vec<AnyValue> {
        self.1
            .get_params()
            .iter()
            .map(|&v| unsafe {
                AnyValue::new_unchecked(v, Raw::from_llvm_type_unchecked(v.get_type()))
            })
            .collect()
    }
}
