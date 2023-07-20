use crate::types::{Type, CreateType, LlvmType};
use crate::dorian::Dorian;
use crate::llvm::types::TypeKind;

/// Represents an anonymous structure.
#[derive(Debug, Copy, Clone)]
pub struct StructData<'a> {
    fields: &'a [&'a dyn Type],
    is_packed: bool
}

impl<'a> StructData<'a> {
    pub fn new(fields: &'a [&'a dyn Type]) -> StructData<'a> {
        StructData { fields, is_packed: false }
    }

    pub fn with_packed(mut self, is_packed: bool) -> Self {
        self.is_packed = is_packed;
        self
    }

    pub fn with_name(self, name: &'a str) -> NamedStructData<'a> {
        NamedStructData { r#struct: self, name }
    }
}

impl<'a> CreateType for StructData<'a> {
    type Type = StructType;

    fn create(self, dorian: &Dorian) -> Self::Type {
        unsafe {
            StructType::from_llvm_type_unchecked(
                dorian.get_context().get_struct_type(
                    self.fields.into_iter()
                        .map(|t| t.get_llvm_type())
                        .collect(),
                    self.is_packed
                )
            )
        }
    }
}

/// Represents a permanent structure type with a name.
#[derive(Debug, Copy, Clone)]
pub struct NamedStructData<'a> {
    r#struct: StructData<'a>,
    name: &'a str,
}

impl<'a> CreateType for NamedStructData<'a> {
    type Type = StructType;

    fn create(self, dorian: &Dorian) -> Self::Type {
        unsafe {
            StructType::from_llvm_type_unchecked(
                dorian.get_context().create_named_struct_type(
                    self.name,
                    self.r#struct.fields.into_iter()
                        .map(|t| t.get_llvm_type())
                        .collect(),
                    self.r#struct.is_packed
                )
            )
        }
    }
}

/// Represents a structure type.
#[derive(Debug, Copy, Clone)]
pub struct StructType(LlvmType);

impl Type for StructType {
    unsafe fn from_llvm_type_unchecked(llvm_type: LlvmType) -> Self {
        Self(llvm_type)
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Struct]
    }

    fn get_llvm_type(&self) -> LlvmType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Struct
    }
}