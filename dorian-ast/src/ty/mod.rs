use crate::function::FunctionType;

mod convert;
mod util;

pub use util::*;

type Bool = bool;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Concrete(ConcreteType),
    Function(FunctionType),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConcreteType {
    Data(DataType),
    Void(VoidType),
}

impl ConcreteType {
    pub fn get_signage(&self) -> Option<Bool> {
        match self {
            ConcreteType::Data(x) => x.get_signage(),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DataType {
    Scalar(ScalarType),
    Vector(VectorType),
}

impl DataType {
    pub fn get_signage(&self) -> Option<Bool> {
        match self {
            DataType::Scalar(ScalarType::Num(NumType::Int(IntType { signed, .. }))) => {
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
    I(u32),
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

// Type-hint retained by values to inform the backend about the type of the value when performing operations.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OperativeType {
    SignedInt,
    UnsignedInt,
    Float,
    Bool,
    Ptr,
    Void,
}
