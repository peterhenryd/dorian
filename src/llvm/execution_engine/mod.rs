pub mod fun;

use crate::llvm::execution_engine::fun::AsBaseFn;
pub use crate::llvm::execution_engine::fun::ExtFn;
use crate::llvm::sys::execution_engine::*;
use crate::llvm::target::Target;
use crate::llvm::{to_c_string, OptimizationLevel};
use crate::module::Module;
use std::mem::{size_of, transmute, MaybeUninit};
use std::ptr::NonNull;

pub struct ExecutionEngine(NonNull<LLVMOpaqueExecutionEngine>);

impl ExecutionEngine {
    pub fn link_in_mc_jit() {
        unsafe {
            LLVMLinkInMCJIT();
        }
    }

    pub fn link_in_interpreter() {
        unsafe {
            LLVMLinkInInterpreter();
        }
    }

    pub fn from_raw(raw: NonNull<LLVMOpaqueExecutionEngine>) -> Self {
        ExecutionEngine(raw)
    }

    pub fn new(module: &Module, optimization_level: OptimizationLevel) -> Self {
        Target::initialize_native_target()
            .expect("the LLVM native target was not initialized successfully");
        Target::initialize_native_asm_printer()
            .expect("the LLVM native ASM printer was not initialized successfully");
        Self::link_in_mc_jit();

        unsafe {
            let mut out = MaybeUninit::uninit();
            let mut err = MaybeUninit::uninit();

            LLVMCreateJITCompilerForModule(
                out.as_mut_ptr(),
                module.get_inner().as_raw().as_ptr(),
                transmute(optimization_level),
                err.as_mut_ptr(),
            );

            ExecutionEngine(NonNull::new_unchecked(out.assume_init()))
        }
    }

    pub fn as_raw(&self) -> NonNull<LLVMOpaqueExecutionEngine> {
        self.0
    }

    pub fn get_fun_address(&self, name: &str) -> Option<usize> {
        unsafe {
            let address = LLVMGetFunctionAddress(self.0.as_ptr(), to_c_string(Some(name)).as_ptr());

            if address == 0 {
                None
            } else {
                Some(address as usize)
            }
        }
    }

    pub fn get_fun<F: AsBaseFn>(&self, name: &str) -> Option<F::BaseFn> {
        assert_eq!(
            size_of::<F::BaseFn>(),
            size_of::<usize>(),
            "The given generic should be the size of a pointer."
        );

        self.get_fun_address(name)
            .map(|address| unsafe { std::mem::transmute_copy(&address) })
    }
}