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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodeGenOptLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RelocMode {
    Default = 0,
    Static = 1,
    PIC = 2,
    DynamicNoPic = 3,
    Ropi = 4,
    Rwpi = 5,
    RopiRwpi = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCodeModel {
    LLVMCodeModelDefault = 0,
    LLVMCodeModelJITDefault = 1,
    LLVMCodeModelTiny = 2,
    LLVMCodeModelSmall = 3,
    LLVMCodeModelKernel = 4,
    LLVMCodeModelMedium = 5,
    LLVMCodeModelLarge = 6,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodeGenFileType {
    Assembly = 0,
    Object = 1,
}

extern "C" {
    pub fn LLVMGetFirstTarget() -> LLVMTargetRef;
    pub fn LLVMGetNextTarget(T: LLVMTargetRef) -> LLVMTargetRef;
    pub fn LLVMGetTargetFromName(Name: *const ::libc::c_char) -> LLVMTargetRef;
    pub fn LLVMGetTargetFromTriple(
        Triple: *const ::libc::c_char,
        T: *mut LLVMTargetRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMGetTargetName(T: LLVMTargetRef) -> *const ::libc::c_char;
    pub fn LLVMGetTargetDescription(T: LLVMTargetRef) -> *const ::libc::c_char;
    pub fn LLVMTargetHasJIT(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMTargetHasTargetMachine(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMTargetHasAsmBackend(T: LLVMTargetRef) -> LLVMBool;
    pub fn LLVMCreateTargetMachine(
        T: LLVMTargetRef,
        Triple: *const ::libc::c_char,
        CPU: *const ::libc::c_char,
        Features: *const ::libc::c_char,
        Level: LLVMCodeGenOptLevel,
        Reloc: LLVMRelocMode,
        CodeModel: LLVMCodeModel,
    ) -> LLVMTargetMachineRef;
    pub fn LLVMDisposeTargetMachine(T: LLVMTargetMachineRef);
    pub fn LLVMGetTargetMachineTarget(T: LLVMTargetMachineRef) -> LLVMTargetRef;
    pub fn LLVMGetTargetMachineTriple(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineCPU(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    pub fn LLVMGetTargetMachineFeatureString(T: LLVMTargetMachineRef) -> *mut ::libc::c_char;
    /// Create a DataLayout based on the target machine.
    pub fn LLVMCreateTargetDataLayout(T: LLVMTargetMachineRef) -> LLVMTargetDataRef;
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
