use inkwell::types::BasicType;
use crate::{llvm, Llvm};
use dorian_ast::function::FunctionType;
use dorian_ast::ty::{
    BoolType, ConcreteType, DataType, FloatType, IntType, IntWidth, NumType, PtrType, ScalarType
    , VectorType, VoidType,
};

impl Llvm {
    fn compile_concrete_type(&self, ty: &ConcreteType) -> llvm::ConcreteType {
        match ty {
            ConcreteType::Data(x) => llvm::ConcreteType::Data(self.compile_data_type(x)),
            ConcreteType::Void(x) => llvm::ConcreteType::Void(self.compile_void_type(x)),
        }
    }

    pub(crate) fn compile_data_type(&self, ty: &DataType) -> llvm::Type {
        match ty {
            DataType::Scalar(x) => self.compile_scalar_type(x).as_basic_type_enum(),
            DataType::Vector(x) => self.compile_vector_type(x).as_basic_type_enum(),
        }
    }

    fn compile_scalar_type(&self, ty: &ScalarType) -> llvm::ScalarType {
        match ty {
            ScalarType::Num(x) => self.compile_num_type(x),
            ScalarType::Bool(x) => llvm::ScalarType::Int(self.compile_bool_type(x)),
            ScalarType::Ptr(x) => llvm::ScalarType::Pointer(self.compile_ptr_type(x)),
        }
    }

    fn compile_num_type(&self, ty: &NumType) -> llvm::ScalarType {
        match ty {
            NumType::Int(x) => llvm::ScalarType::Int(self.compile_int_type(x)),
            NumType::Float(x) => llvm::ScalarType::Float(self.compile_float_type(x)),
        }
    }

    fn compile_int_type(&self, ty: &IntType) -> llvm::IntType {
        match ty.width {
            IntWidth::I(bits) => self.context.custom_width_int_type(bits),
            IntWidth::I8 => self.context.i8_type(),
            IntWidth::I16 => self.context.i16_type(),
            IntWidth::I32 => self.context.i32_type(),
            IntWidth::I64 => self.context.i64_type(),
            IntWidth::I128 => self.context.i128_type(),
        }
    }

    fn compile_float_type(&self, ty: &FloatType) -> llvm::FloatType {
        match ty {
            FloatType::F16 => self.context.f16_type(),
            FloatType::F32 => self.context.f32_type(),
            FloatType::F64 => self.context.f64_type(),
            FloatType::F128 => self.context.f128_type(),
        }
    }

    fn compile_bool_type(&self, _: &BoolType) -> llvm::IntType {
        self.context.bool_type()
    }

    fn compile_ptr_type(&self, _: &PtrType) -> llvm::PointerType {
        self.context.ptr_type(llvm::AddressSpace::default())
    }

    fn compile_void_type(&self, _: &VoidType) -> llvm::VoidType {
        self.context.void_type()
    }

    fn compile_vector_type(&self, ty: &VectorType) -> llvm::VectorType {
        self.compile_scalar_type(&ty.elem).vec_type(ty.len)
    }

    pub(crate) fn compile_function_type(&self, ty: &FunctionType) -> llvm::FunctionType {
        let return_type = self.compile_concrete_type(&ty.return_type);
        let param_types = ty
            .params
            .iter()
            .map(|param_type| self.compile_data_type(&param_type).into())
            .collect::<Vec<_>>();

        return_type.fn_type(&param_types, false)
    }
}