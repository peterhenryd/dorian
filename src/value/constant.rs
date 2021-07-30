use crate::value::Value;

#[derive(Copy, Clone)]
pub struct Const<'a, V: Value<'a>>(&'a V);

impl<'a, V: Value<'a>> Const<'a, V> {
    pub unsafe fn new_unchecked(value: &'a V) -> Self {
        Const(value)
    }

    pub fn get_inner(&self) -> &'a V {
        self.0
    }
}