use crate::ty::DataType;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Global {
    pub name: String,
    pub ty: DataType,
}
