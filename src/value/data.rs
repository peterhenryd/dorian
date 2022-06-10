use crate::fun::block::Block;
use crate::types::Type;
use crate::value::Value;

/// Represents a builder for values.
pub trait BuildValue<'a> {
    /// The type of [Value] that is being built.
    type Value: Value;

    /// Builds the value using a [Block].
    fn build<R: Type>(&self, dorian: &Block<R>) -> Self::Value;
}
