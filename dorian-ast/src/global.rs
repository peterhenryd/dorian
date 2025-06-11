use std::borrow::Cow;
use crate::ty::Type;
use crate::val::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Global<'s> {
    pub name: Cow<'s, str>,
    pub ty: Type,
    pub value: Option<Value<'s>>,
}
