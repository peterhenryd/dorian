pub mod block;

use crate::dorian::Dorian;
use crate::fun::block::Block;
use crate::llvm::fun::Fun as LlvmFun;
use crate::types::{Raw, Type};
use crate::value::any::AnyValue;
use crate::value::Value;
use std::marker::PhantomData;

pub struct Fun<'a, R: Type>(&'a Dorian, LlvmFun<'a>, PhantomData<R>);

impl<'a, R: Type> Fun<'a, R> {
    pub unsafe fn new(dorian: &'a Dorian, llvm_fun: LlvmFun<'a>) -> Fun<'a, R> {
        Fun(dorian, llvm_fun, PhantomData::default())
    }

    pub fn get_inner(&self) -> &LlvmFun<'a> {
        &self.1
    }

    pub fn add_block(&mut self, name: &str) -> Block<'a, R> {
        Block::new(
            self.0,
            self.1.get_context().create_builder(),
            self.1.append_basic_block(name),
        )
    }

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
