use crate::fun::block::Block;
use crate::types::Type;
use crate::value::Value;

pub trait BuildValue<'a> {
    type Value: Value;

    fn build<R: Type>(&self, dorian: &Block<R>) -> Self::Value;
}
