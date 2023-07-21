use inkwell::builder::Builder;
use inkwell::FloatPredicate as CmpOp;
use inkwell::values::FloatValue as InkwellFloatValue;
use crate::fun::block::Block;
use crate::types::float::FloatType;
use crate::types::Type;
use crate::value::data::BuildValue;
use crate::value::{NonAnyValue, Value};

/// Represents a floating-point value.
#[derive(Debug, Copy, Clone)]
pub struct FloatValue<'a>(InkwellFloatValue<'a>, FloatType<'a>);

impl<'a> Value<'a> for FloatValue<'a> {
    type Type = FloatType<'a>;
    type InkwellValue = InkwellFloatValue<'a>;

    unsafe fn new_unchecked(value: Self::InkwellValue, float_type: Self::Type) -> FloatValue<'a> {
        FloatValue(value, float_type)
    }

    fn get_inkwell_value(&self) -> Self::InkwellValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}

impl<'a> NonAnyValue<'a> for FloatValue<'a> {}

/// Represents a binary operation that a floating-point value may undergo.
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

/// Represents a unary operation that a floating-point value may undergo.
pub enum UnaOp {
    Neg,
}

/// Represents a collection of various operations that a floating-point may undergo.
pub enum Float<'a> {
    Bin(BinOp, FloatValue<'a>, FloatValue<'a>),
    Cmp(CmpOp, FloatValue<'a>, FloatValue<'a>),
    Una(UnaOp, FloatValue<'a>),
    Cast(FloatType<'a>, FloatValue<'a>),
}

impl<'a> BuildValue for Float<'a> {
    type Value<'b> = FloatValue<'b>;

    fn build<'b, R: Type<'b>>(&self, block: &'b Block<'b, R>) -> Self::Value<'b> {
        match self {
            Float::Bin(op, lhs, rhs) => {
                let f = match op {
                    BinOp::Add => Builder::build_float_add,
                    BinOp::Sub => Builder::build_float_sub,
                    BinOp::Mul => Builder::build_float_mul,
                    BinOp::Div => Builder::build_float_div,
                    BinOp::Rem => Builder::build_float_rem,
                };

                unsafe {
                    FloatValue::new_unchecked(
                        f(block.get_builder(), lhs.0, rhs.0, ""), // todo: names
                        lhs.get_type().clone(),
                    )
                }
            }
            Float::Cmp(op, lhs, rhs) => unsafe {
                FloatValue::new_unchecked(
                    block.get_builder().build_f_cmp(*op, lhs.0, rhs.0, ""), // todo: names
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
                        val.get_inkwell_value(),
                        t.get_inkwell_type(),
                        None,
                    ),
                    t.clone(),
                )
            },
        }
    }
}
