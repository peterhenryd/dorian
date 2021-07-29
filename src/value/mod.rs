pub mod float;
pub mod int;

use crate::value::int::IntValue;
use crate::types::Type;
use crate::types::kind::TypeKind;

pub trait Value<'a>: Sized {
    unsafe fn from_inner(inner: crate::llvm::value::Value, r#type: Type<'a>) -> Self where Self: Sized;

    fn get_inner(&self) -> crate::llvm::value::Value;

    fn get_type(&self) -> Type<'a>;

    fn as_int_value(&self) -> Option<&IntValue<'a>> {
        if self.get_type().get_data().get_kind() == TypeKind::Int {
            Some(unsafe { std::mem::transmute(self) })
        } else {
            None
        }
    }
}