use crate::types::int::IntType;
use crate::types::ptr::PtrType;
use crate::dorian::Dorian;
use std::fmt::Debug;
use inkwell::types::{AnyType, AnyTypeEnum};

pub mod void;
pub mod float;
pub mod fun;
pub mod int;
pub mod ptr;
pub mod structure;
pub mod array;
pub mod vector;

/// Represents a type that a value has.
pub trait Type<'a>: Debug + Copy + Clone {
    type InkwellType: AnyType<'a>;

    /// Creates a type from an [LlvmType].
    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum<'a>) -> Self
    where
        Self: Sized;

    fn valid_kinds() -> Vec<TypeKind>
    where
        Self: Sized;

    /// Borrow the internal [LlvmType].
    fn get_inkwell_type(&self) -> Self::InkwellType;

    /// Get the LLVM enumerable type that is represented by the implementation.
    fn get_kind(&self) -> TypeKind;

    /// Will return a [Some] of an [IntType] if the [Type] is an underlying integer type.
    fn as_int_type(&self) -> Option<IntType> {
        if let TypeKind::Int = self.get_kind() {
            return Some(unsafe { IntType::from_inkwell_type_unchecked(self.get_inkwell_type()) });
        }

        None
    }

    /// Will return a [Some] of a [PtrType<T>] if the [Type] is an underlying pointer type.
    fn as_ptr_type<T: Type<'a>>(&self) -> Option<PtrType<'a, T>> where Self: Sized {
        if let TypeKind::Ptr = self.get_kind() {
            let ptr = unsafe {
                self.get_inkwell_type().get_pointing_type()
            };

            if T::valid_kinds().contains(&ptr.get_kind()) {
                return Some(unsafe {
                    PtrType::from_inkwell_type_unchecked(
                        self.get_inkwell_type(),
                    )
                });
            }
        }

        None
    }
}

/// Encapsulates a raw [LlvmType] that is not defined at compile-time.
// TODO: is this struct ethical?
#[derive(Debug, Copy, Clone)]
pub struct RawType<'a>(AnyTypeEnum<'a>, TypeKind);

impl<'a> Type<'a> for RawType<'a> {
    type InkwellType = AnyTypeEnum<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum<'a>) -> Self
    where
        Self: Sized,
    {
        RawType(inkwell_type, TypeKind::of_llvm_type(&inkwell_type))
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![
            TypeKind::Void,
            TypeKind::F16,
            TypeKind::F32,
            TypeKind::F64,
            TypeKind::X86F80,
            TypeKind::BF16 ,
            TypeKind::F128,
            TypeKind::PpcF128,
            TypeKind::Label,
            TypeKind::Int,
            TypeKind::Fun,
            TypeKind::Struct,
            TypeKind::Array,
            TypeKind::Ptr,
            TypeKind::Vector,
            TypeKind::Metadata,
            TypeKind::X86Mmx,
            TypeKind::Token,
            TypeKind::ScalableVector,
            TypeKind::X86Amx,
        ]
    }

    fn get_inkwell_type(&self) -> AnyTypeEnum {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        self.1
    }
}

/// Builder for easily creating types.
pub trait CreateType: Clone {
    /// The [Type] being built.
    type Type<'a>: Type<'a>;

    /// Builds the [Type].
    fn create<'a>(self, dorian: &'a Dorian) -> Self::Type<'a>;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeKind {
    Void,
    F16,
    F32,
    F64,
    X86F80,
    BF16 ,
    F128,
    PpcF128,
    Label,
    Int,
    Fun,
    Struct,
    Array,
    Ptr,
    Vector,
    Metadata,
    X86Mmx,
    Token,
    ScalableVector,
    X86Amx,
}