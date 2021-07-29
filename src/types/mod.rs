use crate::types::kind::TypeData;

pub mod kind;

#[derive(Copy, Clone)]
pub struct Type<'a>(crate::llvm::types::Type, &'a dyn TypeData<'a>);

impl<'a> Type<'a> {
    pub fn get_inner(&self) -> crate::llvm::types::Type {
        self.0
    }

    pub fn get_data(&self) -> &'a dyn TypeData<'a> {
        self.1
    }
}