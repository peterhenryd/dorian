use crate::types::data::TypeData;
use crate::dorian::Dorian;
use crate::types::{Type, LlvmType, TypeKind};
use crate::value::int::IntValue;
use crate::value::Value;

#[derive(Copy, Clone)]
pub struct IntType(LlvmType);

impl IntType {
    pub fn const_int(&self, int: u64, sign_extend: bool) -> IntValue {
        unsafe { IntValue::new_unchecked(self.0.const_int(int, sign_extend), *self) }
    }

    pub fn const_int_from_str(&self, str: &str, radix: u8) -> IntValue {
        unsafe { IntValue::new_unchecked(self.0.const_int_from_str(str, radix), *self) }
    }
}

impl<'a> Type<'a> for IntType {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self {
        Self(llvm_type)
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Int
    }
}

#[derive(Copy, Clone)]
pub struct IntData {
    pub bits: u32
}

impl IntData {
    #[inline(always)]
    pub fn new(bits: u32) -> IntData {
        IntData { bits }
    }
}

impl<'a> TypeData<'a> for IntData {
    type Type = IntType;

    #[inline(always)]
    fn create(self, dorian: &Dorian) -> IntType {
        IntType(
            dorian.get_context().get_integer_type(self.bits),
        )
    }
}