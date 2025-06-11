use ast::function::Signature;
use crate::{cl, Cranelift};
use ast::ty::{Type, FloatType, IntWidth, NumType, ScalarType};

impl Cranelift {
    pub(crate) fn compile_type(&self, ty: &Type) -> cl::Type {
        match ty {
            Type::Scalar(x) => self.compile_scalar_type(x),
            Type::Vector(x) => self.compile_scalar_type(&x.elem).by(x.len).unwrap(),
        }
    }
    
    fn compile_scalar_type(&self, ty: &ScalarType) -> cl::Type {
        match ty {
            ScalarType::Num(x) => self.compile_num_type(x),
            ScalarType::Bool(_) => cl::I8,
            ScalarType::Ptr(_) => cl::Type::triple_pointer_type(&self.triple),
        }
    }
    
    fn compile_num_type(&self, num: &NumType) -> cl::Type {
        match num {
            NumType::Int(x) => match x.width {
                IntWidth::I8 => cl::I8,
                IntWidth::I16 => cl::I16,
                IntWidth::I32 => cl::I32,
                IntWidth::I64 => cl::I64,
                IntWidth::I128 => cl::I128,
            }
            NumType::Float(x) => match x {
                FloatType::F16 => cl::F16,
                FloatType::F32 => cl::F32,
                FloatType::F64 => cl::F64,
                FloatType::F128 => cl::F128,
            }
        }
    }
    
    pub(crate) fn compile_signature(&self, signature: &Signature) -> cl::Signature {
        let params = signature.input.iter()
            .map(|x| cl::AbiParam::new(self.compile_type(x)))
            .collect();
        let returns = signature.output.iter()
            .map(|x| cl::AbiParam::new(self.compile_type(x)))
            .collect();
        
        cl::Signature {
            params,
            returns,
            call_conv: cl::CallConv::Fast,
        }
    }
}
