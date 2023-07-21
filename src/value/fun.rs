use crate::value::{NonAnyValue, Value};
use crate::types::fun::FunType;
use std::marker::PhantomData;
use inkwell::values::FunctionValue;

/// Represents a function value.
pub struct FunValue<'a, R: Value<'a>>(FunctionValue<'a>, FunType<'a, R::Type>, PhantomData<R>);

impl<'a, R: Value<'a>> Value<'a> for FunValue<'a, R> {
    type Type = FunType<'a, R::Type>;
    type InkwellValue = FunctionValue<'a>;

    unsafe fn new_unchecked(value: FunctionValue<'a>,  fun_type: Self::Type) -> Self where Self: Sized {
        Self(value, fun_type, PhantomData::default())
    }

    fn get_inkwell_value(&self) -> Self::InkwellValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}

impl<'a, R: Value<'a>> NonAnyValue<'a> for FunValue<'a, R> {}