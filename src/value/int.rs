use crate::llvm;
use crate::value::Value;
use crate::types::Type;

pub struct IntValue<'a>(llvm::value::Value, Type<'a>);

impl<'a> Value<'a> for IntValue<'a> {
    unsafe fn from_inner(inner: llvm::value::Value, r#type: Type<'a>) -> IntValue<'a> {
        IntValue(inner, r#type)
    }

    fn get_inner(&self) -> llvm::value::Value {
        self.0
    }

    fn get_type(&self) -> Type<'a> {
        self.1
    }
}
