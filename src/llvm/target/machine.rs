use crate::llvm::sys::target_machine::{LLVMOpaqueTargetMachine, LLVMCreateTargetDataLayout, LLVMDisposeTargetMachine};
use crate::llvm::target::TargetData;
use std::ptr::NonNull;

pub struct TargetMachine(NonNull<LLVMOpaqueTargetMachine>);

impl TargetMachine {
    pub fn from_raw(raw: NonNull<LLVMOpaqueTargetMachine>) -> TargetMachine {
        TargetMachine(raw)
    }

    pub fn as_raw(&self) -> &NonNull<LLVMOpaqueTargetMachine> {
        &self.0
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

/* TODO: target machine extras
extern "C" {
    pub fn LLVMGetTargetMachineTarget(T: LLVMTargetMachineRef) -> LLVMTargetRef;
    pub fn LLVMGetTargetMachineTriple(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineCPU(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineFeatureString(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    /// Create a DataLayout based on the target machine.
    pub fn LLVMSetTargetMachineAsmVerbosity(T: LLVMTargetMachineRef, VerboseAsm: LLVMBool);
    pub fn LLVMTargetMachineEmitToFile(
        T: LLVMTargetMachineRef,
        M: LLVMModuleRef,
        Filename: *mut ::libc::c_char,
        codegen: LLVMCodeGenFileType,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMTargetMachineEmitToMemoryBuffer(
        T: LLVMTargetMachineRef,
        M: LLVMModuleRef,
        codegen: LLVMCodeGenFileType,
        ErrorMessage: *mut *mut ::libc::c_char,
        OutMemBuf: *mut LLVMMemoryBufferRef,
    ) -> LLVMBool;

    pub fn LLVMGetDefaultTargetTriple() -> *mut ::libc::c_char;
    /// Normalize a target triple. The result needs to be disposed with LLVMDisposeMessage.
    pub fn LLVMNormalizeTargetTriple(triple: *const ::libc::c_char) -> *mut ::libc::c_char;
    /// Get the host CPU as a string. The result needs to be disposed with LLVMDisposeMessage.
    pub fn LLVMGetHostCPUName() -> *mut ::libc::c_char;
    /// Get the host CPU's features as a string. The result needs to be disposed with LLVMDisposeMessage.
    pub fn LLVMGetHostCPUFeatures() -> *mut ::libc::c_char;

    pub fn LLVMAddAnalysisPasses(T: LLVMTargetMachineRef, PM: LLVMPassManagerRef);
}


 */