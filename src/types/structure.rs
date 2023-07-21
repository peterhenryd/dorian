use inkwell::types::{AnyType, AnyTypeEnum, StructType as InkwellStructType};
use crate::types::{Type, CreateType, TypeKind, RawType};
use crate::dorian::Dorian;

/// Represents an anonymous structure.
#[derive(Debug, Clone)]
pub struct StructData<'a> {
    fields: Vec<RawType<'a>>,
    is_packed: bool
}

impl<'a> StructData<'a> {
    pub fn new(fields: Vec<RawType<'a>>) -> StructData<'a> {
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
    type Type = StructType<'a>;

    fn create(self, dorian: &Dorian) -> Self::Type<'_> {
        unsafe {
            StructType::from_inkwell_type_unchecked(
                dorian.get_context().struct_type(
                    self.fields.into_iter()
                        .map(|t| t.get_inkwell_type())
                        .collect(),
                    self.is_packed
                ).as_any_type_enum()
            )
        }
    }
}

/// Represents a permanent structure type with a name.
#[derive(Debug, Clone)]
pub struct NamedStructData<'a> {
    r#struct: StructData<'a>,
    name: &'a str,
}

impl<'a> CreateType for NamedStructData<'a> {
    type Type = StructType<'a>;

    fn create(self, dorian: &Dorian) -> Self::Type<'_> {
        unsafe {
            StructType::from_inkwell_type_unchecked(
                dorian.get_context().create_named_struct_type(
                    self.name,
                    self.r#struct.fields.into_iter()
                        .map(|t| t.get_inkwell_type())
                        .collect(),
                    self.r#struct.is_packed
                )
            )
        }
    }
}

/// Represents a structure type.
#[derive(Debug, Copy, Clone)]
pub struct StructType<'a>(InkwellStructType<'a>);

impl<'a> Type<'a> for StructType<'a> {
    type InkwellType = InkwellStructType<'a>;

    unsafe fn from_inkwell_type_unchecked(inkwell_type: AnyTypeEnum) -> Self {
        Self(inkwell_type.into_struct_type())
    }

    fn valid_kinds() -> Vec<TypeKind> where Self: Sized {
        vec![TypeKind::Struct]
    }

    fn get_inkwell_type(&self) -> InkwellStructType {
        self.0
    }

    fn get_kind(&self) -> TypeKind {
        TypeKind::Struct
    }
}