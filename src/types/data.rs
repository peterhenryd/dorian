use crate::dorian::Dorian;
use crate::types::Type;

pub trait TypeData: Clone {
    type Type: Type;

    fn create(self, dorian: &Dorian) -> Self::Type;
}
