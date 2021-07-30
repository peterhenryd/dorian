use crate::value::Value;
use crate::function::block::Block;

pub trait BuildValue<'a> {
    type Value: Value<'a>;

    fn build(&self, dorian: &Block) -> Self::Value;
}