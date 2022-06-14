use crate::dorian::Dorian;
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::builder::Builder;
use crate::types::Type;
use crate::value::Value;
use std::marker::PhantomData;
use crate::fun::Fun;
use crate::llvm::IntPredicate;
use crate::types::void::VoidType;
use crate::value::any::AnyValue;
use crate::value::cmp::CmpValue;
use crate::value::int::IntValue;

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
        self.1.build_ret(value.get_llvm_value());
    }

    pub fn compare_ints(&mut self, predicate: IntPredicate, lhs: &IntValue, rhs: &IntValue) -> CmpValue {
        unsafe {
            CmpValue::new_inferred(
                self.1.build_i_cmp(predicate, lhs.get_llvm_value(), rhs.get_llvm_value(), None)
            )
        }
    }

    pub fn if_statement(&mut self, cmp: CmpValue, then: &Block<'a, R>, otherwise: &Block<'a, R>) {
        self.1.build_cond_br(cmp.get_llvm_value(), then.2, otherwise.2);
    }

    pub fn call_fun<V: Value>(&mut self, fun: &Fun<V::Type>, args: Vec<&AnyValue>) -> V {
        unsafe {
            V::new_unchecked(
                self.1.build_call(
                    fun.get_return_type().get_llvm_type(),
                    fun.1,
                    args.iter()
                        .map(|value| value.get_llvm_value())
                        .collect(),
                    None),
                fun.get_return_type()
            )
        }
    }

    // TODO: add more block operations
}

impl<'a> Block<'a, VoidType> {
    /// Returns void in the current block, essentially ending the block. Once this is added to a
    /// block, any other operations will be ignored.
    pub fn return_void(&mut self) {
        self.1.build_ret_void();
    }
}