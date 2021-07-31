use crate::llvm::sys::target::{LLVMOpaqueTargetData, LLVMPointerSize, LLVMDisposeTargetData};
use crate::llvm::sys::target_machine::{LLVMGetTargetFromTriple, LLVMTarget};
use crate::llvm::target::triple::TargetTriple;
use crate::llvm::{from_c_string, to_c_string};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

pub mod init;
pub mod machine;
pub mod triple;

pub struct Target<'a>(NonNull<LLVMTarget>, TargetTriple<'a>);

impl<'a> Target<'a> {
    pub fn from_raw(ptr: NonNull<LLVMTarget>, target_triple: TargetTriple<'a>) -> Target<'a> {
        Target(ptr, target_triple)
    }

    pub fn get_ptr(&self) -> NonNull<LLVMTarget> {
        self.0
    }

    pub fn from_triple(target_triple: TargetTriple<'a>) -> Result<Target<'a>, &str> {
        unsafe {
            let mut triple = MaybeUninit::uninit();
            let mut error_message = MaybeUninit::uninit();

            let res = LLVMGetTargetFromTriple(
                to_c_string(Some(target_triple.as_str())).as_ptr(),
                triple.as_mut_ptr(),
                error_message.as_mut_ptr(),
            );

            if res == 0 {
                Err(from_c_string(error_message.assume_init()))
            } else {
                Ok(Target(NonNull::new_unchecked(triple.assume_init()), target_triple))
            }
        }
    }
}

pub struct TargetData(NonNull<LLVMOpaqueTargetData>);

impl TargetData {
    pub fn from_raw(ptr: NonNull<LLVMOpaqueTargetData>) -> TargetData {
        TargetData(ptr)
    }

    pub fn get_ptr(&self) -> NonNull<LLVMOpaqueTargetData> {
        self.0
    }

    pub fn get_ptr_size(&self) -> u32 {
        unsafe { LLVMPointerSize(self.0.as_ptr()) }
    }
}

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetData(self.0.as_ptr());
        }
    }
}