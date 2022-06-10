use crate::value::Value;

/// Represents a value that is evaluated at compile-time.
#[derive(Debug, Copy, Clone)]
pub struct Const<V: Value>(V);

impl<V: Value> Const<V> {
    /// Creates a [Const<V>] without checking if the given argument is actually a constant.
    pub unsafe fn new_unchecked(value: V) -> Self {
        Const(value)
    }

    /// Borrow the internal value.
    pub fn get_val(&self) -> &V {
        &self.0
    }
}
