use inkwell::targets::TargetData;
use inkwell::types::{AnyTypeEnum, IntType as InkwellIntType};
use crate::dorian::Dorian;
use crate::types::{Type, CreateType, TypeKind};
use crate::value::int::IntValue;
use crate::value::Value;
use crate::value::constant::Const;

/// Represents an integer type.
#[derive(Debug, Copy, Clone)]
pub struct IntType<'a>(InkwellIntType<'a>);

impl<'a> IntType<'a> {
    pub fn const_int(&self, int: u64, sign_extend: bool) -> Const<IntValue> {
        unsafe {
            Const::new_unchecked(
                IntValue::new_unchecked(self.0.const_int(int, sign_extend), *self)
            )
        }
    }

    pub fn const_int_from_str(&self, str: &str, radix: u8) -> Const<IntValue> {
        unsafe {
            Const::new_unchecked(
                IntValue::new_unchecked(self.0.const_int_of_string(str, radix), *self)
            )
        }
    }
}

impl<'a> Type<'a> for IntType<'a> {
    type InkwellType = InkwellIntType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum) -> Self {
        Self(inkwell_type.into_int_type())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Int]
    }

    fn get_inkwell_type(&self) -> Self::InkwellType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Int
    }
}

/// Builder for integer type.
#[derive(Debug, Copy, Clone)]
pub enum IntData<'a> {
    Bits(u32),
    Ptr(&'a TargetData),
}

impl<'a> CreateType for IntData<'a> {
    type Type = IntType<'a>;

    #[inline(always)]
    fn create(self, dorian: &Dorian) -> IntType<'a> {
        let bits = match self {
            IntData::Bits(bits) => bits,
            IntData::Ptr(target_data) => target_data.get_ptr_size(),
        };

        unsafe { IntType::from_inkwell_type_unchecked(dorian.get_context().get_integer_type(bits)) }
    }
}
