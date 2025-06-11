use crate::ty::{BoolType, Type, FloatType, IntType, NumType, PtrType, ScalarType, VectorType};

// impl From<...> for Type

impl From<ScalarType> for Type {
    fn from(ty: ScalarType) -> Self {
        Type::Scalar(ty)
    }
}

impl From<NumType> for Type {
    fn from(ty: NumType) -> Self {
        Type::Scalar(ty.into())
    }
}

impl From<IntType> for Type {
    fn from(ty: IntType) -> Self {
        Type::Scalar(ty.into())
    }
}

impl From<FloatType> for Type {
    fn from(ty: FloatType) -> Self {
        Type::Scalar(ty.into())
    }
}

impl From<BoolType> for Type {
    fn from(ty: BoolType) -> Self {
        Type::Scalar(ty.into())
    }
}

impl From<PtrType> for Type {
    fn from(ty: PtrType) -> Self {
        Type::Scalar(ty.into())
    }
}

impl From<VectorType> for Type {
    fn from(ty: VectorType) -> Self {
        Type::Vector(ty)
    }
}

// impl From<...> for ScalarType

impl From<NumType> for ScalarType {
    fn from(ty: NumType) -> Self {
        ScalarType::Num(ty)
    }
}

impl From<IntType> for ScalarType {
    fn from(ty: IntType) -> Self {
        ScalarType::Num(ty.into())
    }
}

impl From<FloatType> for ScalarType {
    fn from(ty: FloatType) -> Self {
        ScalarType::Num(ty.into())
    }
}

impl From<BoolType> for ScalarType {
    fn from(ty: BoolType) -> Self {
        ScalarType::Bool(ty)
    }
}

impl From<PtrType> for ScalarType {
    fn from(ty: PtrType) -> Self {
        ScalarType::Ptr(ty)
    }
}

// impl From<...> for NumType

impl From<IntType> for NumType {
    fn from(ty: IntType) -> Self {
        NumType::Int(ty)
    }
}

impl From<FloatType> for NumType {
    fn from(ty: FloatType) -> Self {
        NumType::Float(ty)
    }
}
