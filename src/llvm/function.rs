use std::ptr::{NonNull};
use crate::llvm::sys::{LLVMValue};
use crate::llvm::sys::core::{LLVMAppendBasicBlockInContext, LLVMCountParams, LLVMGetParam, LLVMGetParams};
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::to_c_string;
use crate::llvm::context::Context;
use crate::llvm::value::Value;
use std::alloc::{alloc, Layout};

pub struct Function<'a>(&'a Context, NonNull<LLVMValue>);

impl<'a> Function<'a> {
    pub fn from_raw(context: &'a Context, function: NonNull<LLVMValue>) -> Function {
        Function(context, function)
    }

    pub fn get_context(&self) -> &'a Context {
        self.0
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

    pub fn get_arity(&self) -> usize {
        (unsafe {
            LLVMCountParams(self.1.as_ptr())
        }) as usize
    }

    pub fn get_params(&self) -> Vec<Value> {
        unsafe {
            let len = self.get_arity();
            let ptr: *mut *mut LLVMValue = alloc(Layout::array::<*mut LLVMValue>(len).unwrap()) as _;
            LLVMGetParams(self.1.as_ptr(), ptr);

            std::slice::from_raw_parts_mut(ptr, len)
                .iter()
                .map(|&v| Value::from_raw(
                    NonNull::new_unchecked(v)
                ))
                .collect()
        }
    }

    pub unsafe fn get_nth_param(&self, n: usize) -> Value {
        Value::from_raw(
            NonNull::new_unchecked(
                LLVMGetParam(self.1.as_ptr(), n as u32)
            )
        )
    }
}
