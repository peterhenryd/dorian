use std::ptr::NonNull;
use crate::llvm::sys::execution_engine::{LLVMOpaqueExecutionEngine, LLVMGetFunctionAddress};
use crate::llvm::to_c_string;

pub struct ExecutionEngine(NonNull<LLVMOpaqueExecutionEngine>);

impl ExecutionEngine {
    pub fn from_raw(raw: NonNull<LLVMOpaqueExecutionEngine>) -> ExecutionEngine {
        ExecutionEngine(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMOpaqueExecutionEngine> {
        self.0
    }

    pub fn get_function_address(&self, name: &str) -> Option<usize> {
        unsafe {
            let address = LLVMGetFunctionAddress(
                self.0.as_ptr(),
                to_c_string(Some(name)).as_ptr()
            );

            if address == 0 {
                None
            } else {
                Some(address as usize)
            }
        }
    }

    pub fn get_function<A, R, F: Fn(A) -> R>(&self, name: &str) -> Option<&F>
    {
        unsafe {
            self.get_function_address(name)
                .map(|address| std::mem::transmute(address))
        }
    }
}