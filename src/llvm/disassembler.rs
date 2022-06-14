use std::ptr::NonNull;
use llvm_sys::disassembler::{LLVMDisasmDispose, LLVMDisasmInstruction, LLVMOpaqueDisasmContext, LLVMSetDisasmOptions};
use crate::llvm::to_c_string;

#[derive(Debug)]
pub struct DisasmContext(NonNull<LLVMOpaqueDisasmContext>);

impl DisasmContext {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMOpaqueDisasmContext>) -> DisasmContext {
        DisasmContext(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMOpaqueDisasmContext> {
        self.0
    }

    // TODO: pub fn create(triple_name: &str, dis_info: *mut ::libc::c_void, tag_type: ::libc::c_int, get_op_info: LLVMOpInfoCallback, symbol_look_up: LLVMSymbolLookupCallback, ) -> DisasmContext;
    // TODO: pub fn create_cpu(triple: *const ::libc::c_char, cpu: *const ::libc::c_char, dis_info: *mut ::libc::c_void, tag_type: ::libc::c_int, get_op_info: LLVMOpInfoCallback, symbol_look_up: LLVMSymbolLookupCallback, ) -> DisasmContext;
    // TODO: pub fn create_cpu_features(Triple: *const ::libc::c_char, CPU: *const ::libc::c_char, Features: *const ::libc::c_char, DisInfo: *mut ::libc::c_void, TagType: ::libc::c_int, GetOpInfo: LLVMOpInfoCallback, SymbolLookUp: LLVMSymbolLookupCallback, ) -> DisasmContext;

    pub fn set_options(&self, options: u64) -> i32 {
        unsafe {
            LLVMSetDisasmOptions(
                self.0.as_ptr(), options
            )
        }
    }

    pub fn instruction(&self, mut bytes: Vec<u8>, pc: u64, out: &str, ) -> usize {
        unsafe {
            LLVMDisasmInstruction(
                self.0.as_ptr(),
                bytes.as_mut_ptr(),
                bytes.len() as u64,
                pc,
                to_c_string(Some(out)).into_raw(),
                out.len(),
            )
        }
    }
}

impl Drop for DisasmContext {
    fn drop(&mut self) {
        unsafe {
            LLVMDisasmDispose(self.0.as_ptr());
        }
    }
}