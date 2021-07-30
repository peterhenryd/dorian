use crate::value::{Value, LlvmValue, AnyValue};
use crate::types::int::IntType;
use crate::value::constant::Const;
use crate::value::data::BuildValue;
use crate::function::block::Block;

#[derive(Copy, Clone)]
pub struct IntValue(LlvmValue, IntType);

impl<'a> Value<'a> for IntValue {
    type Type = IntType;

    unsafe fn new_unchecked(value: LlvmValue, int_type: IntType) -> IntValue {
        IntValue(value, int_type)
    }

    fn get_inner(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &IntType {
        &self.1
    }
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Int<'a> {
    Const(Const<'a, IntValue>),
    Bin(BinOp, &'a IntValue, &'a IntValue),
}

impl<'a> BuildValue<'a> for Int<'a> {
    type Value = IntValue;

    fn build(&self, block: &Block) -> Self::Value {
        match self {
            Int::Const(val) => val.get_inner().clone(),
            Int::Bin(op, lhs, rhs) => {
                unsafe {
                    AnyValue::new_inferred(
                        match op {
                            BinOp::Add => block.get_builder().build_add(
                                lhs.get_inner(),
                                rhs.get_inner(),
                                None,
                            ),
                            BinOp::Sub => block.get_builder().build_sub(
                                lhs.get_inner(),
                                rhs.get_inner(),
                                None,
                            ),
                            BinOp::Mul => block.get_builder().build_mul(
                                lhs.get_inner(),
                                rhs.get_inner(),
                                None,
                            ),
                            BinOp::Div => block.get_builder().build_u_div(
                                lhs.get_inner(),
                                rhs.get_inner(),
                                None,
                            )
                        }
                    )
                }.as_int_value().unwrap()
            }
        }
    }
}