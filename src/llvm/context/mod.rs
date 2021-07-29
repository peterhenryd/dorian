use crate::llvm::sys::core::{LLVMContextCreate, LLVMCreateBuilderInContext, LLVMModuleCreateWithNameInContext, LLVMFunctionType, LLVMContextDispose, LLVMIntTypeInContext};
use crate::llvm::sys::LLVMContext;
use crate::llvm::builder::Builder;
use crate::llvm::module::Module;
use crate::llvm::to_c_string;
use std::ptr::NonNull;
use crate::llvm::types::Type;

pub struct Context(NonNull<LLVMContext>);

impl Context {
    pub fn from_raw(raw: NonNull<LLVMContext>) -> Context {
        Context(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMContext> {
        self.0
    }

    pub fn new() -> Context {
        unsafe { Context(NonNull::new_unchecked(LLVMContextCreate())) }
    }

    pub fn create_builder(&self) -> Builder {
        unsafe {
            Builder::from_raw(NonNull::new_unchecked(LLVMCreateBuilderInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn create_module(&self, name: &str) -> Module {
        unsafe {
            Module::from_raw(self, NonNull::new_unchecked(LLVMModuleCreateWithNameInContext(
                to_c_string(Some(name)).as_ptr(),
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_function_type(&self, parameters: Vec<Type>, return_type: Type, is_var_arg: bool) -> Type {
        let len = parameters.len();
        unsafe {
            Type::from_raw(NonNull::new_unchecked(
                LLVMFunctionType(
                    return_type.as_raw().as_ptr(),
                    parameters.into_iter()
                        .map(|t| t.as_raw().as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    len as u32,
                    is_var_arg as i32,
                )
            ))
        }
    }

    pub fn get_integer_type(&self, bits: u32) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(
                LLVMIntTypeInContext(self.0.as_ptr(), bits)
            ))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.0.as_ptr());
        }
    }
}