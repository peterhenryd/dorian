use crate::fun::block::Block;
use crate::types::Type;
use crate::value::Value;

/// Represents a builder for values.
pub trait BuildValue {
    /// The type of [Value] that is being built.
    type Value<'a>: Value<'a>;

    /// Builds the value using a [Block].
    fn build<'a, R: Type<'a>>(&self, dorian: &'a Block<'a, R>) -> Self::Value<'a>;
}
