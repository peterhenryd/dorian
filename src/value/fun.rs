use crate::value::{LlvmValue, NonAnyValue, Value};
use crate::types::fun::FunType;
use std::marker::PhantomData;

/// REpresents a function value.
pub struct FunValue<R: Value>(LlvmValue, FunType<R::Type>, PhantomData<R>);

impl<R: Value> Value for FunValue<R> {
    type Type = FunType<R::Type>;

    unsafe fn new_unchecked(value: LlvmValue, fun_type: Self::Type) -> Self where Self: Sized {
        Self(value, fun_type, PhantomData::default())
    }

    fn get_llvm_value(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}

impl<R: Value> NonAnyValue for FunValue<R> {}