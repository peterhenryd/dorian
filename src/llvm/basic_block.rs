use crate::llvm::sys::LLVMBasicBlock;
use crate::llvm::value::Value;
use std::ptr::NonNull;
use llvm_sys::core::*;
use crate::llvm::{from_c_string, to_c_string};

#[derive(Debug, Copy, Clone)]
pub struct BasicBlock(NonNull<LLVMBasicBlock>);

impl BasicBlock {
    #[inline(always)]
    pub fn from_raw(raw: NonNull<LLVMBasicBlock>) -> BasicBlock {
        BasicBlock(raw)
    }

    #[inline(always)]
    pub fn as_raw(&self) -> NonNull<LLVMBasicBlock> {
        self.0
    }

    pub fn as_value(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMBasicBlockAsValue(self.0.as_ptr()))
        })
    }

    #[inline(always)]
    pub fn get_name(&self) -> &str {
        unsafe {
            from_c_string(LLVMGetBasicBlockName(self.0.as_ptr()))
        }
    }

    pub fn get_basic_block_parent(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetBasicBlockParent(self.0.as_ptr())
            )
        })
    }

    pub fn get_basic_block_terminator(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetBasicBlockTerminator(self.0.as_ptr())
            )
        })
    }

    pub fn get_next_basic_block(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetNextBasicBlock(self.0.as_ptr())
            )
        })
    }

    pub fn get_previous_basic_block(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetPreviousBasicBlock(self.0.as_ptr())
            )
        })
    }

    pub fn insert_basic_block(&self, name: &str) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMInsertBasicBlock(
                self.0.as_ptr(), to_c_string(Some(name)).as_ptr()
                )
            )
        })
    }

    #[inline(always)]
    pub fn remove_basic_block_from_parent(&self) {
        unsafe { LLVMRemoveBasicBlockFromParent(self.0.as_ptr()); }
    }

    #[inline(always)]
    pub fn move_basic_block_before(&self, basic_block: BasicBlock) {
        unsafe { LLVMMoveBasicBlockBefore(self.0.as_ptr(), basic_block.0.as_ptr()); }
    }

    #[inline(always)]
    pub fn move_basic_block_after(&self, basic_block: BasicBlock) {
        unsafe { LLVMMoveBasicBlockAfter(self.0.as_ptr(), basic_block.0.as_ptr()); }
    }

    pub fn get_first_instruction(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetFirstInstruction(self.0.as_ptr())
            )
        })
    }

    pub fn get_last_instruction(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetLastInstruction(self.0.as_ptr())
            )
        })
    }
}