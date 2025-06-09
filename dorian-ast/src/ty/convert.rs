use crate::function::FunctionType;
use crate::ty::{
    BoolType, ConcreteType, DataType, FloatType, IntType, NumType, PtrType, ScalarType, Type,
    VectorType, VoidType,
};

// impl From<...> for Type

impl From<ConcreteType> for Type {
    fn from(ty: ConcreteType) -> Self {
        Type::Concrete(ty)
    }
}

impl From<DataType> for Type {
    fn from(ty: DataType) -> Self {
        Type::Concrete(ty.into())
    }
}

impl From<FunctionType> for Type {
    fn from(ty: FunctionType) -> Self {
        Type::Function(ty)
    }
}

// impl From<...> for ConcreteType

impl From<DataType> for ConcreteType {
    fn from(ty: DataType) -> Self {
        ConcreteType::Data(ty)
    }
}

impl From<ScalarType> for ConcreteType {
    fn from(ty: ScalarType) -> Self {
        ConcreteType::Data(ty.into())
    }
}

impl From<NumType> for ConcreteType {
    fn from(ty: NumType) -> Self {
        ConcreteType::Data(ty.into())
    }
}

impl From<IntType> for ConcreteType {
    fn from(ty: IntType) -> Self {
        ConcreteType::Data(ty.into())
    }
}

impl From<FloatType> for ConcreteType {
    fn from(ty: FloatType) -> Self {
        ConcreteType::Data(ty.into())
    }
}

impl From<BoolType> for ConcreteType {
    fn from(ty: BoolType) -> Self {
        ConcreteType::Data(ty.into())
    }
}

impl From<PtrType> for ConcreteType {
    fn from(ty: PtrType) -> Self {
        ConcreteType::Data(ty.into())
    }
}

impl From<VoidType> for ConcreteType {
    fn from(ty: VoidType) -> Self {
        ConcreteType::Void(ty)
    }
}

// impl From<...> for DataType

impl From<ScalarType> for DataType {
    fn from(ty: ScalarType) -> Self {
        DataType::Scalar(ty)
    }
}

impl From<NumType> for DataType {
    fn from(ty: NumType) -> Self {
        DataType::Scalar(ty.into())
    }
}

impl From<IntType> for DataType {
    fn from(ty: IntType) -> Self {
        DataType::Scalar(ty.into())
    }
}

impl From<FloatType> for DataType {
    fn from(ty: FloatType) -> Self {
        DataType::Scalar(ty.into())
    }
}

impl From<BoolType> for DataType {
    fn from(ty: BoolType) -> Self {
        DataType::Scalar(ty.into())
    }
}

impl From<PtrType> for DataType {
    fn from(ty: PtrType) -> Self {
        DataType::Scalar(ty.into())
    }
}

impl From<VectorType> for DataType {
    fn from(ty: VectorType) -> Self {
        DataType::Vector(ty)
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
