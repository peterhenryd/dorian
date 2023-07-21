use inkwell::builder::Builder;
use inkwell::values::IntValue as InkwellIntValue;
use crate::fun::block::Block;
use crate::types::int::IntType;
use crate::types::Type;
use crate::value::data::BuildValue;
use crate::value::{NonAnyValue, Value};

/// Represents an integer value.
#[derive(Debug, Copy, Clone)]
pub struct IntValue<'a>(InkwellIntValue<'a>, IntType<'a>);

impl<'a> Value<'a> for IntValue<'a> {
    type Type = IntType<'a>;
    type InkwellValue = InkwellIntValue<'a>;

    unsafe fn new_unchecked(value: Self::InkwellValue, int_type: Self::Type) -> Self {
        IntValue(value, int_type)
    }

    fn get_inkwell_value(&self) -> Self::InkwellValue {
        self.0
    }

    fn get_type(&self) -> &IntType {
        &self.1
    }
}

impl<'a> NonAnyValue<'a> for IntValue<'a> {}

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
    UnsignedDiv,
    SignedDiv,
    ExactSignedDiv,
    UnsignedRem,
    SignedRem,
    Shl,
    Shr,
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
    Bin(BinOp, &'a IntValue<'a>, &'a IntValue<'a>),
    Una(UnaOp, &'a IntValue<'a>),
}

impl<'a> BuildValue for Int<'a> {
    type Value<'b> = IntValue<'b>;

    fn build<'b, R: Type<'b>>(&self, block: &'b Block<'b, R>) -> Self::Value<'b> {
        match self {
            Int::Bin(op, lhs, rhs) => {
                let f = match op {
                    BinOp::Add => Builder::build_int_add,
                    BinOp::NuwAdd => Builder::build_int_nuw_add,
                    BinOp::NswAdd => Builder::build_int_nsw_add,
                    BinOp::Sub => Builder::build_int_sub,
                    BinOp::NuwSub => Builder::build_int_nuw_sub,
                    BinOp::NswSub => Builder::build_int_nsw_sub,
                    BinOp::Mul => Builder::build_int_mul,
                    BinOp::NuwMul => Builder::build_int_nuw_mul,
                    BinOp::NswMul => Builder::build_int_nsw_mul,
                    BinOp::UnsignedDiv => Builder::build_int_unsigned_div,
                    BinOp::SignedDiv => Builder::build_int_signed_div,
                    BinOp::ExactSignedDiv => Builder::build_int_exact_signed_div,
                    BinOp::UnsignedRem => Builder::build_int_unsigned_rem,
                    BinOp::SignedRem => Builder::build_int_signed_rem,
                    BinOp::Shl => Builder::build_left_shift,
                    BinOp::Shr => Builder::build_right_shift,
                    BinOp::And => Builder::build_and,
                    BinOp::Or => Builder::build_or,
                    BinOp::Xor => Builder::build_xor,
                };

                unsafe {
                    IntValue::new_unchecked(
                        f(
                            block.get_builder(),
                            lhs.get_inkwell_value(),
                            rhs.get_inkwell_value(),
                            "" // TODO: names
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
                        f(block.get_builder(), val.get_inkwell_value(), ""), //todo: names
                        *val.get_type(),
                    )
                }
            }
        }
    }
}
