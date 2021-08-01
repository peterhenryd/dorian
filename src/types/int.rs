use crate::dorian::Dorian;
use crate::llvm::types::TypeKind;
use crate::types::{LlvmType, Type, CreateType};
use crate::value::int::IntValue;
use crate::value::Value;
use crate::llvm::target::TargetData;

#[derive(Debug, Copy, Clone)]
pub struct IntType(LlvmType);

impl IntType {
    pub fn const_int(&self, int: u64, sign_extend: bool) -> IntValue {
        unsafe { IntValue::new_unchecked(self.0.const_int(int, sign_extend), *self) }
    }

    pub fn const_int_from_str(&self, str: &str, radix: u8) -> IntValue {
        unsafe { IntValue::new_unchecked(self.0.const_int_from_str(str, radix), *self) }
    }
}

impl Type for IntType {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self {
        Self(llvm_type)
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Int]
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Int
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IntData<'a> {
    Bits(u32),
    Ptr(&'a TargetData),
}

impl CreateType for IntData<'_> {
    type Type = IntType;

    #[inline(always)]
    fn create(self, dorian: &Dorian) -> IntType {
        let bits = match self {
            IntData::Bits(bits) => bits,
            IntData::Ptr(target_data) => target_data.get_ptr_size(),
        };

        unsafe { IntType::from_llvm_type_unchecked(dorian.get_context().get_integer_type(bits)) }
    }
}
