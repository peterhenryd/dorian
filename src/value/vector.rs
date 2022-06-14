use crate::types::vector::VectorType;
use crate::value::{LlvmValue, Value};

pub struct VectorValue<V: Value>(LlvmValue, VectorType<V::Type>);


pub fn d() {
}