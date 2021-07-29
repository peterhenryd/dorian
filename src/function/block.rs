use crate::llvm::basic_block::BasicBlock;

pub struct Block(BasicBlock);

impl Block {
    pub fn from_llvm(llvm: BasicBlock) -> Block {
        Block(llvm)
    }

    pub fn get_llvm(&self) -> BasicBlock {
        self.0
    }
}