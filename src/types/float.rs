use crate::dorian::Dorian;
use crate::llvm::context::Context;
use crate::llvm::types::TypeKind;
use crate::types::data::TypeData;
use crate::types::{LlvmType, Type};

#[derive(Copy, Clone)]
pub struct FloatType(LlvmType);

impl Type for FloatType {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self {
        Self(llvm_type)
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        self.0.get_kind()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FloatData {
    F16 = 1,
    BF16 = 18,
    F32 = 2,
    F64 = 3,
    X86F80 = 4,
    F128 = 5,
    PpcF128 = 6,
}

impl TypeData for FloatData {
    type Type = FloatType;

    #[inline(always)]
    fn create(self, dorian: &Dorian) -> FloatType {
        let f = match self {
            FloatData::F16 => Context::get_f16_type,
            FloatData::F32 => Context::get_f32_type,
            FloatData::F64 => Context::get_f64_type,
            FloatData::X86F80 => Context::get_x86_f80_type,
            FloatData::BF16 => Context::get_bf16_type,
            FloatData::F128 => Context::get_f128_type,
            FloatData::PpcF128 => Context::get_ppc_f128_type,
        };

        unsafe { FloatType::from_llvm_type_unchecked(f(dorian.get_context())) }
    }
}
