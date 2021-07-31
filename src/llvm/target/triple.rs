use crate::llvm::from_c_string;
use crate::llvm::sys::target_machine::LLVMGetDefaultTargetTriple;

pub struct TargetTriple<'a>(&'a str);

impl<'a> TargetTriple<'a> {
    pub fn default() -> TargetTriple<'a> {
        unsafe { TargetTriple(from_c_string(LLVMGetDefaultTargetTriple())) }
    }

    pub fn from_str(str: &'a str) -> TargetTriple<'a> {
        TargetTriple(str)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}