use crate::dorian::Dorian;
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::builder::Builder;
use crate::types::Type;
use crate::value::Value;
use std::marker::PhantomData;
use crate::types::void::VoidType;

pub struct Block<'a, R: Type>(&'a Dorian, Builder, BasicBlock, PhantomData<&'a R>);

impl<'a, R: Type> Block<'a, R> {
    pub fn new(dorian: &'a Dorian, builder: Builder, basic_block: BasicBlock) -> Block<'a, R> {
        builder.position_at_end(&basic_block);
        Block(dorian, builder, basic_block, PhantomData::default())
    }

    pub fn get_dorian(&self) -> &'a Dorian {
        self.0
    }

    pub fn get_builder(&self) -> &Builder {
        &self.1
    }

    pub fn get_basic_block(&self) -> BasicBlock {
        self.2
    }

    pub fn return_value(&mut self, value: &impl Value<Type = R>) {
        unsafe {
            self.1.build_ret(value.get_llvm_value());
        }
    }
}

impl<'a> Block<'a, VoidType> {
    pub fn return_void(&mut self) {
        unsafe {
            self.1.build_ret_void();
        }
    }
}