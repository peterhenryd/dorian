use std::borrow::Cow;
use crate::ty::Type;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Struct<'s> {
    pub name: Cow<'s, str>,
    pub fields: Vec<Type>,
}
