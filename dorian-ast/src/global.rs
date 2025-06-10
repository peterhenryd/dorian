use std::borrow::Cow;
use crate::ty::DataType;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Global<'s> {
    pub name: Cow<'s, str>,
    pub ty: DataType,
}
