use crate::llvm::basic_block::BasicBlock;
use crate::llvm::builder::Builder;
use crate::value::Value;

pub struct Block(Builder, BasicBlock);

impl Block {
    pub fn from_llvm(builder: Builder, basic_block: BasicBlock) -> Block {
        builder.position_at_end(&basic_block);
        Block(builder, basic_block)
    }

    pub fn get_builder(&self) -> &Builder {
        &self.0
    }

    pub fn get_basic_block(&self) -> BasicBlock {
        self.1
    }

    pub fn return_value<'a>(&mut self, value: &impl Value<'a>) {
        self.0.build_ret(value.get_inner());
    }
}