use crate::val::{Arg, Bin, BinOp, Call, Lit, Una, UnaOp, Value};

macro_rules! bin_op {
    ($($function:ident => $variant:ident),+ $(,)?) => {
        $(
        pub fn $function<T: From<Bin>>(lhs: Value, rhs: Value) -> T {
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

pub fn neg<T: From<Una>>(value: Value) -> T {
    T::from(Una {
        operand: value,
        op: UnaOp::Neg,
        no_wrap: false,
    })
}

pub fn not<T: From<Una>>(value: Value) -> T {
    T::from(Una {
        operand: value,
        op: UnaOp::Not,
        no_wrap: false,
    })
}

pub fn arg<T: From<Arg>>(param_index: u32) -> T {
    T::from(Arg { param_index })
}

pub fn lit<T: Into<Lit>, U: From<Lit>>(value: T) -> U {
    U::from(value.into())
}

pub fn call<T: From<Call>>(function_name: impl Into<String>, args: impl Into<Vec<Value>>) -> T {
    T::from(Call {
        function_name: function_name.into(),
        args: args.into(),
    })
}
