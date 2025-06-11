mod convert;
pub mod util;

type Bool = bool;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Scalar(ScalarType),
    Vector(VectorType),
}

impl Type {
    pub fn get_signage(&self) -> Option<Bool> {
        match self {
            Type::Scalar(ScalarType::Num(NumType::Int(IntType { signed, .. }))) => {
                Some(*signed)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScalarType {
    Num(NumType),
    Bool(BoolType),
    Ptr(PtrType),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum NumType {
    Int(IntType),
    Float(FloatType),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct IntType {
    pub width: IntWidth,
    pub signed: bool,
}

impl IntType {
    pub const S8: Self = IntType {
        width: IntWidth::I8,
        signed: true,
    };
    pub const S16: Self = IntType {
        width: IntWidth::I16,
        signed: true,
    };
    pub const S32: Self = IntType {
        width: IntWidth::I32,
        signed: true,
    };
    pub const S64: Self = IntType {
        width: IntWidth::I64,
        signed: true,
    };
    pub const S128: Self = IntType {
        width: IntWidth::I128,
        signed: true,
    };
    pub const U8: Self = IntType {
        width: IntWidth::I8,
        signed: false,
    };
    pub const U16: Self = IntType {
        width: IntWidth::I16,
        signed: false,
    };
    pub const U32: Self = IntType {
        width: IntWidth::I32,
        signed: false,
    };
    pub const U64: Self = IntType {
        width: IntWidth::I64,
        signed: false,
    };
    pub const U128: Self = IntType {
        width: IntWidth::I128,
        signed: false,
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum IntWidth {
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum FloatType {
    F16,
    F32,
    F64,
    F128,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BoolType;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PtrType;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VectorType {
    pub elem: ScalarType,
    pub len: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VoidType;