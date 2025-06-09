mod convert;
mod util;

pub use util::*;

#[derive(Debug)]
pub enum Value {
    Context(ContextValue),
    Expr(Box<Expr>),
    Lit(Lit),
    Call(Call),
}

#[derive(Debug)]
pub enum ContextValue {
    Arg(Arg),
    Var(Var),
}

#[derive(Debug)]
pub struct Arg {
    pub param_index: u32,
}

#[derive(Debug)]
pub struct Var {
    pub scope_index: usize,
    pub item_index: usize,
}

#[derive(Debug)]
pub enum Expr {
    Bin(Bin),
    Una(Una),
}

#[derive(Debug)]
pub struct Bin {
    pub lhs: Value,
    pub op: BinOp,
    pub rhs: Value,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Una {
    pub operand: Value,
    pub op: UnaOp,
}

#[derive(Debug)]
pub enum UnaOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum Lit {
    Num(Num),
    Bool(bool),
}

#[derive(Debug, Copy, Clone)]
pub enum Num {
    Int(Int),
    Float(Float),
}

#[derive(Debug, Copy, Clone)]
pub enum Int {
    Signed(SignedInt),
    Unsigned(UnsignedInt),
}

#[derive(Debug, Copy, Clone)]
pub enum SignedInt {
    B8(i8),
    B16(i16),
    B32(i32),
    B64(i64),
    B128(i128),
}

#[derive(Debug, Copy, Clone)]
pub enum UnsignedInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

#[derive(Debug, Copy, Clone)]
pub enum Float {
    F32(f32),
    F64(f64),
}

#[derive(Debug)]
pub struct Call {
    pub function_name: String,
    pub args: Vec<Value>,
}
