use crate::val::{
    Arg, Bin, Call, ContextValue, Expr, Float, Int, Lit, Num, SignedInt, Una, UnsignedInt, Value,
    Var,
};

// impl From<...> for Value

impl From<ContextValue> for Value {
    fn from(value: ContextValue) -> Self {
        Value::Context(value)
    }
}

impl From<Arg> for Value {
    fn from(value: Arg) -> Self {
        Value::Context(value.into())
    }
}

impl From<Var> for Value {
    fn from(value: Var) -> Self {
        Value::Context(value.into())
    }
}

impl From<Expr> for Value {
    fn from(value: Expr) -> Self {
        Value::Expr(Box::new(value))
    }
}

impl From<Bin> for Value {
    fn from(value: Bin) -> Self {
        Value::Expr(Box::new(value.into()))
    }
}

impl From<Una> for Value {
    fn from(value: Una) -> Self {
        Value::Expr(Box::new(value.into()))
    }
}

impl From<Lit> for Value {
    fn from(value: Lit) -> Self {
        Value::Lit(value)
    }
}

impl From<Num> for Value {
    fn from(value: Num) -> Self {
        Value::Lit(value.into())
    }
}

impl From<Int> for Value {
    fn from(value: Int) -> Self {
        Value::Lit(value.into())
    }
}

impl From<SignedInt> for Value {
    fn from(value: SignedInt) -> Self {
        Value::Lit(value.into())
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Lit(value.into())
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Lit(value.into())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Lit(value.into())
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Lit(value.into())
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Lit(value.into())
    }
}

impl From<UnsignedInt> for Value {
    fn from(value: UnsignedInt) -> Self {
        Value::Lit(value.into())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Lit(value.into())
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Lit(value.into())
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Lit(value.into())
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Lit(value.into())
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::Lit(value.into())
    }
}

impl From<Float> for Value {
    fn from(value: Float) -> Self {
        Value::Lit(value.into())
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Lit(value.into())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Lit(value.into())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Lit(value.into())
    }
}

impl From<Call> for Value {
    fn from(value: Call) -> Self {
        Value::Call(value)
    }
}

// impl From<...> for ContextValue

impl From<Arg> for ContextValue {
    fn from(value: Arg) -> Self {
        ContextValue::Arg(value)
    }
}

impl From<Var> for ContextValue {
    fn from(value: Var) -> Self {
        ContextValue::Var(value)
    }
}

// impl From<...> for Expr

impl From<Bin> for Expr {
    fn from(value: Bin) -> Self {
        Expr::Bin(value)
    }
}

impl From<Una> for Expr {
    fn from(value: Una) -> Self {
        Expr::Una(value)
    }
}

// impl From<...> for Lit

impl From<Num> for Lit {
    fn from(value: Num) -> Self {
        Lit::Num(value)
    }
}

impl From<Int> for Lit {
    fn from(value: Int) -> Self {
        Lit::Num(value.into())
    }
}

impl From<SignedInt> for Lit {
    fn from(value: SignedInt) -> Self {
        Lit::Num(value.into())
    }
}

impl From<i8> for Lit {
    fn from(value: i8) -> Self {
        Lit::Num(value.into())
    }
}

impl From<i16> for Lit {
    fn from(value: i16) -> Self {
        Lit::Num(value.into())
    }
}

impl From<i32> for Lit {
    fn from(value: i32) -> Self {
        Lit::Num(value.into())
    }
}

impl From<i64> for Lit {
    fn from(value: i64) -> Self {
        Lit::Num(value.into())
    }
}

impl From<i128> for Lit {
    fn from(value: i128) -> Self {
        Lit::Num(value.into())
    }
}

impl From<UnsignedInt> for Lit {
    fn from(value: UnsignedInt) -> Self {
        Lit::Num(value.into())
    }
}

impl From<u8> for Lit {
    fn from(value: u8) -> Self {
        Lit::Num(value.into())
    }
}

impl From<u16> for Lit {
    fn from(value: u16) -> Self {
        Lit::Num(value.into())
    }
}

impl From<u32> for Lit {
    fn from(value: u32) -> Self {
        Lit::Num(value.into())
    }
}

impl From<u64> for Lit {
    fn from(value: u64) -> Self {
        Lit::Num(value.into())
    }
}

impl From<u128> for Lit {
    fn from(value: u128) -> Self {
        Lit::Num(value.into())
    }
}

impl From<Float> for Lit {
    fn from(value: Float) -> Self {
        Lit::Num(value.into())
    }
}

impl From<f32> for Lit {
    fn from(value: f32) -> Self {
        Lit::Num(value.into())
    }
}

impl From<f64> for Lit {
    fn from(value: f64) -> Self {
        Lit::Num(value.into())
    }
}

impl From<bool> for Lit {
    fn from(value: bool) -> Self {
        Lit::Bool(value)
    }
}

// impl From<...> for Num

impl From<Int> for Num {
    fn from(value: Int) -> Self {
        Num::Int(value)
    }
}

impl From<SignedInt> for Num {
    fn from(value: SignedInt) -> Self {
        Num::Int(value.into())
    }
}

impl From<i8> for Num {
    fn from(value: i8) -> Self {
        Num::Int(value.into())
    }
}

impl From<i16> for Num {
    fn from(value: i16) -> Self {
        Num::Int(value.into())
    }
}

impl From<i32> for Num {
    fn from(value: i32) -> Self {
        Num::Int(value.into())
    }
}

impl From<i64> for Num {
    fn from(value: i64) -> Self {
        Num::Int(value.into())
    }
}

impl From<i128> for Num {
    fn from(value: i128) -> Self {
        Num::Int(value.into())
    }
}

impl From<UnsignedInt> for Num {
    fn from(value: UnsignedInt) -> Self {
        Num::Int(value.into())
    }
}

impl From<u8> for Num {
    fn from(value: u8) -> Self {
        Num::Int(value.into())
    }
}

impl From<u16> for Num {
    fn from(value: u16) -> Self {
        Num::Int(value.into())
    }
}

impl From<u32> for Num {
    fn from(value: u32) -> Self {
        Num::Int(value.into())
    }
}

impl From<u64> for Num {
    fn from(value: u64) -> Self {
        Num::Int(value.into())
    }
}

impl From<u128> for Num {
    fn from(value: u128) -> Self {
        Num::Int(value.into())
    }
}

impl From<Float> for Num {
    fn from(value: Float) -> Self {
        Num::Float(value)
    }
}

impl From<f32> for Num {
    fn from(value: f32) -> Self {
        Num::Float(value.into())
    }
}

impl From<f64> for Num {
    fn from(value: f64) -> Self {
        Num::Float(value.into())
    }
}

// impl From<...> for Int

impl From<SignedInt> for Int {
    fn from(value: SignedInt) -> Self {
        Int::Signed(value)
    }
}

impl From<i8> for Int {
    fn from(value: i8) -> Self {
        Int::Signed(value.into())
    }
}

impl From<i16> for Int {
    fn from(value: i16) -> Self {
        Int::Signed(value.into())
    }
}

impl From<i32> for Int {
    fn from(value: i32) -> Self {
        Int::Signed(value.into())
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Int::Signed(value.into())
    }
}

impl From<i128> for Int {
    fn from(value: i128) -> Self {
        Int::Signed(value.into())
    }
}

impl From<UnsignedInt> for Int {
    fn from(value: UnsignedInt) -> Self {
        Int::Unsigned(value)
    }
}

impl From<u8> for Int {
    fn from(value: u8) -> Self {
        Int::Unsigned(value.into())
    }
}

impl From<u16> for Int {
    fn from(value: u16) -> Self {
        Int::Unsigned(value.into())
    }
}

impl From<u32> for Int {
    fn from(value: u32) -> Self {
        Int::Unsigned(value.into())
    }
}

impl From<u64> for Int {
    fn from(value: u64) -> Self {
        Int::Unsigned(value.into())
    }
}

impl From<u128> for Int {
    fn from(value: u128) -> Self {
        Int::Unsigned(value.into())
    }
}

// impl From<...> for Float

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float::F32(value)
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float::F64(value)
    }
}

// impl From<...> for SignedInt

impl From<i8> for SignedInt {
    fn from(value: i8) -> Self {
        SignedInt::B8(value)
    }
}

impl From<i16> for SignedInt {
    fn from(value: i16) -> Self {
        SignedInt::B16(value)
    }
}

impl From<i32> for SignedInt {
    fn from(value: i32) -> Self {
        SignedInt::B32(value)
    }
}

impl From<i64> for SignedInt {
    fn from(value: i64) -> Self {
        SignedInt::B64(value)
    }
}

impl From<i128> for SignedInt {
    fn from(value: i128) -> Self {
        SignedInt::B128(value)
    }
}

// impl From<...> for UnsignedInt

impl From<u8> for UnsignedInt {
    fn from(value: u8) -> Self {
        UnsignedInt::U8(value)
    }
}

impl From<u16> for UnsignedInt {
    fn from(value: u16) -> Self {
        UnsignedInt::U16(value)
    }
}

impl From<u32> for UnsignedInt {
    fn from(value: u32) -> Self {
        UnsignedInt::U32(value)
    }
}

impl From<u64> for UnsignedInt {
    fn from(value: u64) -> Self {
        UnsignedInt::U64(value)
    }
}

impl From<u128> for UnsignedInt {
    fn from(value: u128) -> Self {
        UnsignedInt::U128(value)
    }
}
