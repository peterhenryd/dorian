use inkwell::values::VectorValue as InkwellVectorValue;
use crate::types::vector::VectorType;
use crate::value::{Value};

pub struct VectorValue<'a, V: Value<'a>>(InkwellVectorValue<'a>, VectorType<'a, V::Type>);