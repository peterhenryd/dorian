use crate::llvm::sys::target_machine::{LLVMCreateTargetMachine, LLVMOpaqueTargetMachine, LLVMCreateTargetDataLayout, LLVMDisposeTargetMachine};
use crate::llvm::target::{Target, TargetData};
use std::ptr::NonNull;
use crate::llvm::{to_c_string, OptimizationLevel, RelocMode, CodeModel};
use std::mem::transmute;
pub struct TargetMachine(NonNull<LLVMOpaqueTargetMachine>);

impl TargetMachine {
    pub fn new(target: Target, cpu: &str, features: &str, optimization_level: OptimizationLevel, reloc_mode: RelocMode, code_model: CodeModel) -> TargetMachine {
        TargetMachine(unsafe {
            NonNull::new_unchecked(
                LLVMCreateTargetMachine(
                    target.0.as_ptr(),
                    to_c_string(Some(target.1.as_str())).as_ptr(),
                    to_c_string(Some(cpu)).as_ptr(),
                    to_c_string(Some(features)).as_ptr(),
                    transmute(optimization_level),
                    transmute(reloc_mode),
                    transmute(code_model),
                )
            )
        })
    }

    pub fn get_data(&self) -> TargetData {
        TargetData::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateTargetDataLayout(self.0.as_ptr())
            )
        })
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetMachine(self.0.as_ptr());
        }
    }
}