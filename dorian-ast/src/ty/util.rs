use crate::ty::{
    BoolType, FloatType, IntType, IntWidth, PtrType, ScalarType, VectorType, VoidType,
};

// Signed integer type utility functions

pub fn signed_int<T: From<IntType>>(width: u32) -> T {
    T::from(IntType {
        width: IntWidth::I(width),
        signed: true,
    })
}

pub fn s8<T: From<IntType>>() -> T {
    T::from(IntType::S8)
}

pub fn s16<T: From<IntType>>() -> T {
    T::from(IntType::S16)
}

pub fn s32<T: From<IntType>>() -> T {
    T::from(IntType::S32)
}

pub fn s64<T: From<IntType>>() -> T {
    T::from(IntType::S64)
}

pub fn s128<T: From<IntType>>() -> T {
    T::from(IntType::S128)
}

// Unsigned integer type utility functions

pub fn unsigned_int<T: From<IntType>>(width: u32) -> T {
    T::from(IntType {
        width: IntWidth::I(width),
        signed: false,
    })
}

pub fn u8<T: From<IntType>>() -> T {
    T::from(IntType::U8)
}

pub fn u16<T: From<IntType>>() -> T {
    T::from(IntType::U16)
}

pub fn u32<T: From<IntType>>() -> T {
    T::from(IntType::U32)
}

pub fn u64<T: From<IntType>>() -> T {
    T::from(IntType::U64)
}

pub fn u128<T: From<IntType>>() -> T {
    T::from(IntType::U128)
}

// Floating-point type utility functions

pub fn f16<T: From<FloatType>>() -> T {
    T::from(FloatType::F16)
}

pub fn f32<T: From<FloatType>>() -> T {
    T::from(FloatType::F32)
}

pub fn f64<T: From<FloatType>>() -> T {
    T::from(FloatType::F64)
}

pub fn f128<T: From<FloatType>>() -> T {
    T::from(FloatType::F128)
}

// Miscellaneous type utility functions

pub fn bool<T: From<BoolType>>() -> T {
    T::from(BoolType)
}

pub fn ptr<T: From<PtrType>>() -> T {
    T::from(PtrType)
}

pub fn vector<T: From<VectorType>>(elem: impl Into<ScalarType>, len: u32) -> T {
    T::from(VectorType {
        elem: elem.into(),
        len,
    })
}

pub fn void<T: From<VoidType>>() -> T {
    T::from(VoidType)
}
