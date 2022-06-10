use crate::fun::block::Block;
use crate::llvm::builder::Builder;
use crate::types::int::IntType;
use crate::types::Type;
use crate::value::data::BuildValue;
use crate::value::{LlvmValue, Value};

/// Represents an integer value.
#[derive(Debug, Copy, Clone)]
pub struct IntValue(LlvmValue, IntType);

impl Value for IntValue {
    type Type = IntType;

    unsafe fn new_unchecked(value: LlvmValue, int_type: IntType) -> IntValue {
        IntValue(value, int_type)
    }

    fn get_llvm_value(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &IntType {
        &self.1
    }
}

/// Represents a binary operation that an integer value may undergo.
pub enum BinOp {
    Add,
    NuwAdd,
    NswAdd,
    Sub,
    NuwSub,
    NswSub,
    Mul,
    NuwMul,
    NswMul,
    UDiv,
    SDiv,
    ExactUDiv,
    ExactSDiv,
    URem,
    SRem,
    Shl,
    LShr,
    AShr,
    And,
    Or,
    Xor,
}

/// Represents a unary operation that a integer value may undergo.
pub enum UnaOp {
    Not,
    Neg,
    NswNeg,
    NuwNeg,
}

/// Represents a collection of various operations that a integer may undergo.
pub enum Int<'a> {
    Bin(BinOp, &'a IntValue, &'a IntValue),
    Una(UnaOp, &'a IntValue),
}

impl<'a> BuildValue<'a> for Int<'a> {
    type Value = IntValue;

    fn build<R: Type>(&self, block: &Block<R>) -> Self::Value {
        match self {
            Int::Bin(op, lhs, rhs) => {
                let f = match op {
                    BinOp::Add => Builder::build_add,
                    BinOp::NuwAdd => Builder::build_nuw_add,
                    BinOp::NswAdd => Builder::build_nsw_add,
                    BinOp::Sub => Builder::build_sub,
                    BinOp::NuwSub => Builder::build_nuw_sub,
                    BinOp::NswSub => Builder::build_nsw_sub,
                    BinOp::Mul => Builder::build_mul,
                    BinOp::NuwMul => Builder::build_nuw_mul,
                    BinOp::NswMul => Builder::build_nsw_mul,
                    BinOp::UDiv => Builder::build_u_div,
                    BinOp::SDiv => Builder::build_s_div,
                    BinOp::ExactUDiv => Builder::build_exact_u_div,
                    BinOp::ExactSDiv => Builder::build_exact_s_div,
                    BinOp::URem => Builder::build_u_rem,
                    BinOp::SRem => Builder::build_s_rem,
                    BinOp::Shl => Builder::build_shl,
                    BinOp::LShr => Builder::build_l_shr,
                    BinOp::AShr => Builder::build_a_shr,
                    BinOp::And => Builder::build_and,
                    BinOp::Or => Builder::build_or,
                    BinOp::Xor => Builder::build_xor,
                };

                unsafe {
                    IntValue::new_unchecked(
                        f(
                            block.get_builder(),
                            lhs.get_llvm_value(),
                            rhs.get_llvm_value(),
                            None,
                        ),
                        *lhs.get_type(),
                    )
                }
            }
            Int::Una(op, val) => {
                let f = match op {
                    UnaOp::Not => Builder::build_not,
                    UnaOp::Neg => Builder::build_neg,
                    UnaOp::NswNeg => Builder::build_nsw_neg,
                    UnaOp::NuwNeg => Builder::build_nuw_neg,
                };

                unsafe {
                    IntValue::new_unchecked(
                        f(block.get_builder(), val.get_llvm_value(), None),
                        *val.get_type(),
                    )
                }
            }
        }
    }
}
