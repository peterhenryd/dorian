use crate::dorian::Dorian;
use crate::types::Type;
use crate::value::Value;
use std::marker::PhantomData;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::IntPredicate;
use inkwell::values::AnyValue;
use crate::fun::Fun;
use crate::types::void::VoidType;
use crate::value::cmp::CmpValue;
use crate::value::int::IntValue;

/// Represents a block of operations in a function.
pub struct Block<'a, R: Type<'a>> {
    dorian: &'a Dorian,
    builder: Builder<'a>,
    basic_block: BasicBlock<'a>,
    return_type: R,
}

impl<'a, R: Type<'a>> Block<'a, R> {
    /// Creates a [Block] instance and aligns the builder to the block.
    pub fn new(dorian: &'a Dorian, builder: Builder, basic_block: BasicBlock, return_type: R) -> Block<'a, R> {
        builder.position_at_end(basic_block);
        Block { dorian, builder, basic_block, return_type }
    }

    /// Borrow the internal [Dorian] instance.
    pub fn get_dorian(&self) -> &'a Dorian {
        self.dorian
    }

    /// Borrow the internal [Builder] for more complex operations.
    pub fn get_builder(&self) -> &Builder {
        &self.builder
    }

    /// Borrow the internal [BasicBlock] for more complex operations.
    pub fn get_basic_block(&self) -> BasicBlock {
        self.basic_block
    }

    /// Return a value in the current block. Once this is added to a block, any other operations
    /// added will be ignored.
    pub fn return_value(&mut self, value: &impl Value<'a, Type = R>) {
        self.builder.build_return(Some(&value.get_inkwell_value()));
    }

    pub fn compare_ints(&mut self, predicate: IntPredicate, lhs: &IntValue, rhs: &IntValue, name: &str) -> CmpValue {
        unsafe {
            CmpValue::new_inferred(
                self.builder.build_int_compare(predicate, lhs.get_inkwell_value(), rhs.get_inkwell_value(), name)
                    .as_any_value_enum()
            )
        }
    }

    pub fn if_statement(&mut self, cmp: CmpValue, then: &Block<'a, R>, otherwise: &Block<'a, R>) {
        self.builder.build_cond_br(cmp.get_inkwell_value(), then.basic_block, otherwise.basic_block);
    }

    pub fn call_fun<V: Value<'a>>(&mut self, fun: &Fun<'a, V::Type>, args: Vec<&AnyValue>) -> V {
        unsafe {
            V::new_unchecked(
                self.builder.build_call(
                    fun.inkwell_function_value,
                    args.iter()
                        .map(|value| value.get_inkwell_value())
                        .collect(), "").as_any_value_enum(),
                fun.get_return_type()
            )
        }
    }

    // TODO: add more block operations
}

impl<'a> Block<'a, VoidType<'a>> {
    /// Returns void in the current block, essentially ending the block. Once this is added to a
    /// block, any other operations will be ignored.
    pub fn return_void(&mut self) {
        self.builder.build_ret_void();
    }
}