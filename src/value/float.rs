use crate::fun::block::Block;
use crate::llvm::builder::Builder;
pub use crate::llvm::FloatPredicate as CmpOp;
use crate::types::float::FloatType;
use crate::types::Type;
use crate::value::data::BuildValue;
use crate::value::{LlvmValue, Value};

#[derive(Debug, Copy, Clone)]
pub struct FloatValue(LlvmValue, FloatType);

impl Value for FloatValue {
    type Type = FloatType;

    unsafe fn new_unchecked(value: LlvmValue, float_type: FloatType) -> FloatValue {
        FloatValue(value, float_type)
    }

    fn get_llvm_value(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &FloatType {
        &self.1
    }
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

pub enum UnaOp {
    Neg,
}

pub enum Float<'a> {
    Bin(BinOp, &'a FloatValue, &'a FloatValue),
    Cmp(CmpOp, &'a FloatValue, &'a FloatValue),
    Una(UnaOp, &'a FloatValue),
    Cast(FloatType, &'a FloatValue),
}

impl<'a> BuildValue<'a> for Float<'a> {
    type Value = FloatValue;

    fn build<R: Type>(&self, block: &Block<R>) -> Self::Value {
        match self {
            Float::Bin(op, lhs, rhs) => {
                let f = match op {
                    BinOp::Add => Builder::build_f_add,
                    BinOp::Sub => Builder::build_f_sub,
                    BinOp::Mul => Builder::build_f_mul,
                    BinOp::Div => Builder::build_f_div,
                    BinOp::Rem => Builder::build_f_rem,
                };

                unsafe {
                    FloatValue::new_unchecked(
                        f(block.get_builder(), lhs.0, rhs.0, None),
                        lhs.get_type().clone(),
                    )
                }
            }
            Float::Cmp(op, lhs, rhs) => unsafe {
                FloatValue::new_unchecked(
                    block.get_builder().build_f_cmp(*op, lhs.0, rhs.0, None),
                    lhs.get_type().clone(),
                )
            },
            Float::Una(op, val) => {
                let f = match op {
                    UnaOp::Neg => Builder::build_f_neg,
                };

                unsafe {
                    FloatValue::new_unchecked(
                        f(block.get_builder(), val.0, None),
                        val.get_type().clone(),
                    )
                }
            }
            Float::Cast(t, val) => unsafe {
                FloatValue::new_unchecked(
                    block.get_builder().build_fp_cast(
                        val.get_llvm_value(),
                        t.get_llvm_type(),
                        None,
                    ),
                    t.clone(),
                )
            },
        }
    }
}
