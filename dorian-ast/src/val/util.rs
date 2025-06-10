use std::borrow::Cow;
use crate::val::{Arg, Bin, BinOp, Call, Lit, Una, UnaOp, Value, Var};

macro_rules! bin_op {
    ($($function:ident => $variant:ident),+ $(,)?) => {
        $(
        pub fn $function<'s, T: From<Bin<'s>>>(lhs: Value<'s>, rhs: Value<'s>) -> T {
                T::from(Bin {
                    lhs,
                    rhs,
                    op: BinOp::$variant,
                    no_wrap: false,
                })
            }
        )+
    };
}

bin_op! {
    add => Add,
    sub => Sub,
    mul => Mul,
    div => Div,
    rem => Rem,
    and => And,
    or => Or,
    bit_and => BitAnd,
    bit_or => BitOr,
    bit_xor => BitXor,
    shl => Shl,
    shr => Shr,
    eq => Eq,
    ne => Ne,
    lt => Lt,
    gt => Gt,
    le => Le,
    ge => Ge,
}

pub fn neg<'s, T: From<Una<'s>>>(value: Value<'s>) -> T {
    T::from(Una {
        operand: value,
        op: UnaOp::Neg,
        no_wrap: false,
    })
}

pub fn not<'s, T: From<Una<'s>>>(value: Value<'s>) -> T {
    T::from(Una {
        operand: value,
        op: UnaOp::Not,
        no_wrap: false,
    })
}

pub fn arg<T: From<Arg>>(param_index: u32) -> T {
    T::from(Arg { param_index })
}

pub fn var<'s, T: From<Var<'s>>>(name: impl Into<Cow<'s, str>>) -> T {
    T::from(Var { 
        name: name.into(),
    })
}

pub fn lit<T: Into<Lit>, U: From<Lit>>(value: T) -> U {
    U::from(value.into())
}

pub fn call<'s, T: From<Call<'s>>>(function_name: impl Into<Cow<'s, str>>, args: impl Into<Vec<Value<'s>>>) -> T {
    T::from(Call {
        function_name: function_name.into(),
        args: args.into(),
    })
}
