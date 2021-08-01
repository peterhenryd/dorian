use crate::llvm::sys::LLVMBasicBlock;
use std::ptr::NonNull;

#[derive(Debug, Copy, Clone)]
pub struct BasicBlock(NonNull<LLVMBasicBlock>);

impl BasicBlock {
    pub fn from_raw(raw: NonNull<LLVMBasicBlock>) -> BasicBlock {
        BasicBlock(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMBasicBlock> {
        self.0
    }
}
