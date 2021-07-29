use crate::llvm::sys::target_machine::{LLVMTarget, LLVMGetTargetFromTriple};
use std::ptr::NonNull;
use crate::llvm::target::triple::TargetTriple;
use crate::llvm::{to_c_string, from_c_string};
use std::mem::MaybeUninit;

pub mod init;
pub mod triple;

pub struct Target(NonNull<LLVMTarget>);

impl Target {
    pub fn from_raw(raw: NonNull<LLVMTarget>) -> Target {
        Target(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMTarget> {
        self.0
    }

    pub fn from_triple(target_triple: TargetTriple) -> Result<Target, &str> {
        unsafe {
            let triple = MaybeUninit::uninit();
            let error_message = MaybeUninit::uninit();

            let res = LLVMGetTargetFromTriple(
                to_c_string(Some(target_triple.as_str())).as_ptr(),
                triple.assume_init(),
                error_message.assume_init()
            );

            if res == 0 {
                Err(from_c_string(*error_message.assume_init()))
            } else {
                Ok(Target(NonNull::new_unchecked(*triple.assume_init())))
            }
        }
    }
}
