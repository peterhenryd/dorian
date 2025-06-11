pub use cranelift::{
    codegen::{
        ir::{
            types::*,
            AbiParam,
            Value as Scalar,
            Inst,
            Function,
            FuncRef,
            InstBuilder,
            UserExternalName,
            ExtFuncData,
            ExternalName,
            Signature,
            UserFuncName,
            condcodes::{
                FloatCC as FloatCmpOp,
                IntCC as IntCmpOp,
            },
        },
        isa::{
            CallConv,
        }
    },
    frontend::{
        FunctionBuilder,
        FunctionBuilderContext,
    },
    module::{
        ModuleDeclarations as Module,
        Linkage,
        FuncId,
        FuncOrDataId,
    },
};

pub struct Value {
    pub raw: ValueItem,
    pub signage: Option<bool>,
}

pub enum ValueItem {
    Scalar(Scalar),
    Variable(Inst)
}

impl ValueItem {
    pub(crate) fn to_scalar(&self) -> Option<Scalar> {
        match self {
            Self::Scalar(scalar) => Some(*scalar),
            _ => None,
        }
    }
}