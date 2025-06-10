use crate::ty::DataType;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<DataType>,
}
