use crate::dorian::Dorian;
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::builder::Builder;
use crate::types::Type;
use crate::value::Value;
use std::marker::PhantomData;
use crate::types::void::VoidType;

/// Represents a block of operations in a function.
pub struct Block<'a, R: Type>(&'a Dorian, Builder, BasicBlock, PhantomData<&'a R>);

impl<'a, R: Type> Block<'a, R> {
    /// Creates a [Block] instance and aligns the builder to the block.
    pub fn new(dorian: &'a Dorian, builder: Builder, basic_block: BasicBlock) -> Block<'a, R> {
        builder.position_at_end(&basic_block);
        Block(dorian, builder, basic_block, PhantomData::default())
    }

    /// Borrow the internal [Dorian] instance.
    pub fn get_dorian(&self) -> &'a Dorian {
        self.0
    }

    /// Borrow the internal [Builder] for more complex operations.
    pub fn get_builder(&self) -> &Builder {
        &self.1
    }

    /// Borrow the internal [BasicBlock] for more complex operations.
    pub fn get_basic_block(&self) -> BasicBlock {
        self.2
    }

    /// Return a value in the current block. Once this is added to a block, any other operations
    /// added will be ignored.
    pub fn return_value(&mut self, value: &impl Value<Type = R>) {
        unsafe {
            self.1.build_ret(value.get_llvm_value());
        }
    }

    // TODO: add more block operations
}

impl<'a> Block<'a, VoidType> {
    /// Returns void in the current block, essentially ending the block. Once this is added to a
    /// block, any other operations will be ignored.
    pub fn return_void(&mut self) {
        unsafe {
            self.1.build_ret_void();
        }
    }
}