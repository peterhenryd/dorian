use crate::types::Type;
use crate::dorian::Dorian;

pub trait TypeData<'a>: Clone {
    type Type: Type<'a>;

    fn create(self, dorian: &Dorian) -> Self::Type;
}