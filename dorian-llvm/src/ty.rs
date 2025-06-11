use crate::{llvm, Llvm};
use ast::ty::{BoolType, FloatType, IntType, IntWidth, NumType, PtrType, ScalarType, Type, VectorType};
use inkwell::types::BasicType;
use ast::function::Signature;

impl Llvm {
    pub(crate) fn compile_type(&self, ty: &Type) -> llvm::Type {
        match ty {
            Type::Scalar(x) => self.compile_scalar_type(x).as_basic_type_enum(),
            Type::Vector(x) => self.compile_vector_type(x).as_basic_type_enum(),
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

    fn compile_vector_type(&self, ty: &VectorType) -> llvm::VectorType {
        self.compile_scalar_type(&ty.elem).vec_type(ty.len)
    }

    pub(crate) fn compile_signature(&self, signature: &Signature) -> llvm::FunctionType {
        let param_types = signature.input.iter()
            .map(|param| self.compile_type(param).into())
            .collect::<Vec<_>>();
        let return_type = self.compile_aggregate_type(&signature.output);

        return_type.fn_type(&param_types, false)
    }
    
    fn compile_aggregate_type(&self, types: &[Type]) -> llvm::AggregateType {
        match types.len() {
            0 => llvm::AggregateType::Void(self.context.void_type()),
            1 => llvm::AggregateType::Value(self.compile_type(&types[0])),
            _ => {
                let field_types = types.iter()
                    .map(|x| self.compile_type(x))
                    .collect::<Vec<_>>();
                let struct_type = self.context.struct_type(&field_types, false);
                
                llvm::AggregateType::Struct(struct_type)
            }
        }
    }
}