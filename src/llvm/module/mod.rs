use crate::llvm::sys::core::*;
use crate::llvm::sys::LLVMModule;
use crate::llvm::context::Context;
use crate::llvm::target::triple::TargetTriple;
use crate::llvm::types::Type;
use crate::llvm::{from_c_string, to_c_string};
use std::ptr::NonNull;
use crate::llvm::function::Function;

pub struct Module<'a>(&'a Context, NonNull<LLVMModule>);

impl<'a> Module<'a> {
    #[inline]
    pub fn from_raw(context: &'a Context, module: NonNull<LLVMModule>) -> Module<'a> {
        Module(context, module)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMModule> {
        self.1
    }

    pub fn set_source_file_name(&mut self, name: &str) {
        unsafe {
            LLVMSetSourceFileName(
                self.1.as_ptr(),
                to_c_string(Some(name)).as_ptr(),
                name.len() + 1,
            );
        }
    }

    pub fn get_data_layout(&self) -> &str {
        unsafe { from_c_string(LLVMGetDataLayoutStr(self.1.as_ptr())) }
    }

    pub fn set_data_layout(&mut self, data_layout: &str) {
        unsafe { LLVMSetDataLayout(self.1.as_ptr(), to_c_string(Some(data_layout)).as_ptr()) }
    }

    pub fn get_target_triple(&self) -> TargetTriple {
        TargetTriple::from_str(unsafe { from_c_string(LLVMGetTarget(self.1.as_ptr())) })
    }

    pub fn set_target(&mut self, target_triple: TargetTriple) {
        unsafe {
            LLVMSetTarget(
                self.1.as_ptr(),
                to_c_string(Some(target_triple.as_str())).as_ptr(),
            )
        }
    }

    pub fn get_context(&self) -> &Context {
        self.0
    }

    pub unsafe fn add_function(&mut self, name: &str, function_type: Type) -> Function {
        Function::from_raw(self.0, NonNull::new_unchecked(LLVMAddFunction(
            self.1.as_ptr(),
            to_c_string(Some(name)).as_ptr(),
            function_type.as_raw().as_ptr(),
        )))
    }
}

impl ToString for Module<'_> {
    fn to_string(&self) -> String {
        unsafe { from_c_string(LLVMPrintModuleToString(self.1.as_ptr())).to_string() }
    }
}

impl Drop for Module<'_> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.1.as_ptr());
        }
    }
}

impl Clone for Module<'_> {
    fn clone(&self) -> Self {
        unsafe { Module::from_raw(self.0, NonNull::new_unchecked(LLVMCloneModule(self.1.as_ptr()))) }
    }
}
