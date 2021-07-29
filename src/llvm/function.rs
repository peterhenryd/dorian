use std::ptr::NonNull;
use crate::llvm::sys::{LLVMValue};
use crate::llvm::sys::core::LLVMAppendBasicBlockInContext;
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::to_c_string;
use crate::llvm::context::Context;

pub struct Function<'a>(&'a Context, NonNull<LLVMValue>);

impl<'a> Function<'a> {
    pub fn from_raw(context: &'a Context, function: NonNull<LLVMValue>) -> Function {
        Function(context, function)
    }

    pub fn as_raw(&self) -> NonNull<LLVMValue> {
        self.1
    }

    pub fn append_basic_block(&mut self, name: &str) -> BasicBlock {
        unsafe {
            BasicBlock::from_raw(NonNull::new_unchecked(
                LLVMAppendBasicBlockInContext(
                    self.0.as_raw().as_ptr(),
                    self.1.as_ptr(),
                    to_c_string(Some(name)).as_ptr()
                )
            ))
        }
    }
}
