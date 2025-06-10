pub(crate) use inkwell::{
    attributes::{Attribute, AttributeLoc},
    basic_block::BasicBlock as Block,
    builder::Builder,
    context::Context,
    module::Module,
    types::{
        BasicMetadataTypeEnum as MetadataType,
        BasicTypeEnum as Type,
        FloatType,
        FunctionType,
        IntType,
        PointerType,
        StringRadix,
        VectorType,
        VoidType,
    },
    values::{
        BasicValueEnum as RawValue,
        FloatValue as Float,
        FunctionValue as Function,
        IntValue as Int
    },
    AddressSpace,
    FloatPredicate as FloatCmpOp,
    IntPredicate as IntCmpOp,
};
use inkwell::types::BasicType;

#[derive(Copy, Clone)]
pub(crate) enum ConcreteType<'ctx> {
    Data(Type<'ctx>),
    Void(VoidType<'ctx>),
}

impl<'ctx> ConcreteType<'ctx> {
    pub(crate) fn fn_type(
        &self,
        param_types: &[MetadataType<'ctx>],
        is_var_arg: bool,
    ) -> FunctionType<'ctx> {
        match self {
            ConcreteType::Data(x) => x.fn_type(param_types, is_var_arg),
            ConcreteType::Void(x) => x.fn_type(param_types, is_var_arg),
        }
    }
}

pub(crate) enum ScalarType<'ctx> {
    Int(IntType<'ctx>),
    Float(FloatType<'ctx>),
    Pointer(PointerType<'ctx>),
}

impl<'ctx> ScalarType<'ctx> {
    pub(crate) fn vec_type(&self, size: u32) -> VectorType<'ctx> {
        match self {
            ScalarType::Int(x) => x.vec_type(size),
            ScalarType::Float(x) => x.vec_type(size),
            ScalarType::Pointer(x) => x.vec_type(size),
        }
    }

    pub(crate) fn as_basic_type_enum(&self) -> Type<'ctx> {
        match self {
            ScalarType::Int(x) => x.as_basic_type_enum(),
            ScalarType::Float(x) => x.as_basic_type_enum(),
            ScalarType::Pointer(x) => x.as_basic_type_enum(),
        }
    }
}



// An LLVM value with AST context information.
#[derive(Copy, Clone)]
pub(crate) struct Value<'ctx> {
    pub(crate) raw: RawValue<'ctx>,
    pub(crate) signage: Option<bool>,
}

impl<'ctx> Value<'ctx> {
    pub(crate) fn new(raw: RawValue<'ctx>, signage: Option<bool>) -> Self {
        Value { raw, signage }
    }
}