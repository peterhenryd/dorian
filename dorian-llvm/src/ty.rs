use crate::Llvm;
use dorian_ast::function::FunctionType;
use dorian_ast::ty::{
    BoolType, ConcreteType, DataType, FloatType, IntType, IntWidth, NumType, PtrType, ScalarType,
    Type, VectorType, VoidType,
};
use inkwell::AddressSpace;
use inkwell::types::{
    AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum,
    FloatType as LlvmFloatType, FunctionType as LlvmFunctionType, IntType as LlvmIntType,
    PointerType as LlvmPointerType, VectorType as LlvmVectorType, VoidType as LlvmVoidType,
};

impl Llvm {
    fn compile_type(&self, ty: &Type) -> AnyTypeEnum {
        match ty {
            Type::Concrete(x) => self.compile_concrete_type(x).as_any_type_enum(),
            Type::Function(x) => self.compile_function_type(x).as_any_type_enum(),
        }
    }

    fn compile_concrete_type(&self, ty: &ConcreteType) -> LlvmConcreteType {
        match ty {
            ConcreteType::Data(x) => LlvmConcreteType::Data(self.compile_data_type(x)),
            ConcreteType::Void(x) => LlvmConcreteType::Void(self.compile_void_type(x)),
        }
    }

    pub(crate) fn compile_data_type(&self, ty: &DataType) -> BasicTypeEnum {
        match ty {
            DataType::Scalar(x) => self.compile_scalar_type(x).as_basic_type_enum(),
            DataType::Vector(x) => self.compile_vector_type(x).as_basic_type_enum(),
        }
    }

    fn compile_scalar_type(&self, ty: &ScalarType) -> LlvmScalarType {
        match ty {
            ScalarType::Num(x) => self.compile_num_type(x),
            ScalarType::Bool(x) => LlvmScalarType::Int(self.compile_bool_type(x)),
            ScalarType::Ptr(x) => LlvmScalarType::Pointer(self.compile_ptr_type(x)),
        }
    }

    fn compile_num_type(&self, ty: &NumType) -> LlvmScalarType {
        match ty {
            NumType::Int(x) => LlvmScalarType::Int(self.compile_int_type(x)),
            NumType::Float(x) => LlvmScalarType::Float(self.compile_float_type(x)),
        }
    }

    fn compile_int_type(&self, ty: &IntType) -> LlvmIntType {
        match ty.width {
            IntWidth::I(bits) => self.context.custom_width_int_type(bits),
            IntWidth::I8 => self.context.i8_type(),
            IntWidth::I16 => self.context.i16_type(),
            IntWidth::I32 => self.context.i32_type(),
            IntWidth::I64 => self.context.i64_type(),
            IntWidth::I128 => self.context.i128_type(),
        }
    }

    fn compile_float_type(&self, ty: &FloatType) -> LlvmFloatType {
        match ty {
            FloatType::F16 => self.context.f16_type(),
            FloatType::F32 => self.context.f32_type(),
            FloatType::F64 => self.context.f64_type(),
            FloatType::F128 => self.context.f128_type(),
        }
    }

    fn compile_bool_type(&self, _: &BoolType) -> LlvmIntType {
        self.context.bool_type()
    }

    fn compile_ptr_type(&self, _: &PtrType) -> LlvmPointerType {
        self.context.ptr_type(AddressSpace::default())
    }

    fn compile_void_type(&self, _: &VoidType) -> LlvmVoidType {
        self.context.void_type()
    }

    fn compile_vector_type(&self, ty: &VectorType) -> LlvmVectorType {
        self.compile_scalar_type(&ty.elem).vec_type(ty.len)
    }

    pub(crate) fn compile_function_type(&self, ty: &FunctionType) -> LlvmFunctionType {
        let return_type = self.compile_concrete_type(&ty.return_type);
        let param_types = ty
            .params
            .iter()
            .map(|param_type| self.compile_data_type(&param_type).into())
            .collect::<Vec<_>>();

        return_type.fn_type(&param_types, false)
    }
}

enum LlvmConcreteType<'ctx> {
    Data(BasicTypeEnum<'ctx>),
    Void(LlvmVoidType<'ctx>),
}

impl<'ctx> LlvmConcreteType<'ctx> {
    fn fn_type(
        &self,
        param_types: &[BasicMetadataTypeEnum<'ctx>],
        is_var_arg: bool,
    ) -> LlvmFunctionType<'ctx> {
        match self {
            LlvmConcreteType::Data(x) => x.fn_type(param_types, is_var_arg),
            LlvmConcreteType::Void(x) => x.fn_type(param_types, is_var_arg),
        }
    }

    fn as_any_type_enum(&self) -> AnyTypeEnum<'ctx> {
        match self {
            LlvmConcreteType::Data(x) => x.as_any_type_enum(),
            LlvmConcreteType::Void(x) => x.as_any_type_enum(),
        }
    }
}

enum LlvmScalarType<'ctx> {
    Int(LlvmIntType<'ctx>),
    Float(LlvmFloatType<'ctx>),
    Pointer(LlvmPointerType<'ctx>),
}

impl<'ctx> LlvmScalarType<'ctx> {
    fn vec_type(&self, size: u32) -> LlvmVectorType<'ctx> {
        match self {
            LlvmScalarType::Int(x) => x.vec_type(size),
            LlvmScalarType::Float(x) => x.vec_type(size),
            LlvmScalarType::Pointer(x) => x.vec_type(size),
        }
    }

    fn as_basic_type_enum(&self) -> BasicTypeEnum<'ctx> {
        match self {
            LlvmScalarType::Int(x) => x.as_basic_type_enum(),
            LlvmScalarType::Float(x) => x.as_basic_type_enum(),
            LlvmScalarType::Pointer(x) => x.as_basic_type_enum(),
        }
    }
}
