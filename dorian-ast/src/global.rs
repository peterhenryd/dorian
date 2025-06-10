use std::borrow::Cow;
use crate::ty::DataType;
use crate::val::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Global<'s> {
    pub name: Cow<'s, str>,
    pub ty: DataType,
    pub value: Option<Value<'s>>,
}
