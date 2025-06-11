mod convert;
pub mod util;

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'s> {
    Context(ContextValue<'s>),
    Expr(Box<Expr<'s>>),
    Lit(Lit),
    Call(Call<'s>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContextValue<'s> {
    Arg(Arg),
    Var(Var<'s>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Arg {
    pub param_index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'s> {
    pub name: Cow<'s, str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'s> {
    Bin(Bin<'s>),
    Una(Una<'s>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bin<'s> {
    pub lhs: Value<'s>,
    pub rhs: Value<'s>,
    pub op: BinOp,
    pub no_wrap: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinOp {
    // Arithmetic operators
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    // Short-circuit logical operators
    And,
    Or,
    // Bitwise operators
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    // Comparison operators
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Una<'s> {
    pub operand: Value<'s>,
    pub op: UnaOp,
    pub no_wrap: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnaOp {
    Neg,
    Not,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Lit {
    Num(Num),
    Bool(bool),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Num {
    Int(Int),
    Float(Float),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Int {
    Signed(SignedInt),
    Unsigned(UnsignedInt),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SignedInt {
    B8(i8),
    B16(i16),
    B32(i32),
    B64(i64),
    B128(i128),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum UnsignedInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl UnsignedInt {
    pub fn cast_signed(self) -> SignedInt {
        match self {
            UnsignedInt::U8(x) => SignedInt::B8(x as i8),
            UnsignedInt::U16(x) => SignedInt::B16(x as i16),
            UnsignedInt::U32(x) => SignedInt::B32(x as i32),
            UnsignedInt::U64(x) => SignedInt::B64(x as i64),
            UnsignedInt::U128(x) => SignedInt::B128(x as i128),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Float {
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call<'s> {
    pub function_name: Cow<'s, str>,
    pub args: Vec<Value<'s>>,
}
