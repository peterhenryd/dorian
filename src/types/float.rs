use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, FloatType as InkwellFloatType};
use crate::dorian::Dorian;
use crate::types::{Type, CreateType, TypeKind};

/// Represents a floating-point type.
#[derive(Debug, Copy, Clone)]
pub struct FloatType<'a>(InkwellFloatType<'a>);

impl<'a> Type<'a> for FloatType<'a> {
    type InkwellType = InkwellFloatType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum) -> Self {
        Self(inkwell_type.into_float_type())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![
            TypeKind::F16,
            TypeKind::F32,
            TypeKind::F64,
            TypeKind::X86F80,
            TypeKind::BF16,
            TypeKind::F128,
            TypeKind::PpcF128,
        ]
    }

    fn get_inkwell_type(&self) -> Self::InkwellType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        self.0.get_kind()
    }
}

/// Represents a kind of floating-point.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum FloatData {
    F16 = 1,
    F32 = 2,
    F64 = 3,
    X86F80 = 4,
    F128 = 5,
    PpcF128 = 6,
}

impl CreateType for FloatData {
    type Type<'a> = FloatType<'a>;

    #[inline(always)]
    fn create<'a>(self, dorian: &'a Dorian) -> Self::Type<'a> {
        let f = match self {
            FloatData::F16 => Context::f16_type,
            FloatData::F32 => Context::f32_type,
            FloatData::F64 => Context::f64_type,
            FloatData::X86F80 => Context::x86_f80_type,
            FloatData::F128 => Context::f128_type,
            FloatData::PpcF128 => Context::ppc_f128_type,
        };

        unsafe { FloatType::from_inkwell_type_unchecked(f(dorian.get_context())) }
    }
}
