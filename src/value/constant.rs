use crate::value::Value;

#[derive(Debug, Copy, Clone)]
pub struct Const<V: Value>(V);

impl<V: Value> Const<V> {
    pub unsafe fn new_unchecked(value: V) -> Self {
        Const(value)
    }

    pub fn get_val(&self) -> &V {
        &self.0
    }
}
