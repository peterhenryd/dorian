use crate::ty::DataType;

pub struct Struct {
    pub name: String,
    pub fields: Vec<DataType>,
}
