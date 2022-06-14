use crate::llvm::basic_block::BasicBlock;
use crate::llvm::context::Context;
use crate::llvm::sys::core::{
    LLVMAppendBasicBlockInContext, LLVMCountParams, LLVMGetParam, LLVMGetParams,
};
use crate::llvm::sys::LLVMValue;
use crate::llvm::{from_c_string, to_c_string, VerifierFailureAction};
use crate::llvm::value::Value;
use std::alloc::{alloc, Layout};
use std::mem::transmute;
use std::ptr::NonNull;
use llvm_sys::analysis::{LLVMVerifyFunction, LLVMViewFunctionCFG, LLVMViewFunctionCFGOnly};
use llvm_sys::core::{LLVMCountBasicBlocks, LLVMDeleteFunction, LLVMGetBasicBlocks, LLVMGetEntryBasicBlock, LLVMGetFirstBasicBlock, LLVMGetFunctionCallConv, LLVMGetGC, LLVMGetIntrinsicID, LLVMGetLastBasicBlock, LLVMGetNextFunction, LLVMGetPersonalityFn, LLVMGetPreviousFunction, LLVMHasPersonalityFn, LLVMIsFunctionVarArg, LLVMSetFunctionCallConv, LLVMSetGC, LLVMSetPersonalityFn};
use llvm_sys::debuginfo::{LLVMGetSubprogram, LLVMSetSubprogram};
use llvm_sys::LLVMBasicBlock;
use crate::llvm::debug::Metadata;
use crate::llvm::types::Type;
use crate::types::LlvmType;

#[derive(Clone, Copy)]
pub struct Fun<'a>(&'a Context, NonNull<LLVMValue>, LlvmType);

impl<'a> Fun<'a> {
    pub fn from_raw(context: &'a Context, fun: NonNull<LLVMValue>, ty: LlvmType) -> Fun<'a> {
        Fun(context, fun, ty)
    }

    pub fn get_context(&self) -> &'a Context {
        self.0
    }

    pub fn as_raw(&self) -> NonNull<LLVMValue> {
        self.1
    }

    pub fn append_basic_block(&mut self, name: &str) -> BasicBlock {
        unsafe {
            BasicBlock::from_raw(NonNull::new_unchecked(LLVMAppendBasicBlockInContext(
                self.0.as_raw().as_ptr(),
                self.1.as_ptr(),
                to_c_string(Some(name)).as_ptr(),
            )))
        }
    }

    pub fn get_arity(&self) -> usize {
        unsafe { LLVMCountParams(self.1.as_ptr()) as usize }
    }

    pub fn get_params(&self) -> Vec<Value> {
        unsafe {
            let len = self.get_arity();
            let ptr: *mut *mut LLVMValue =
                alloc(Layout::array::<*mut LLVMValue>(len).unwrap()) as _;
            LLVMGetParams(self.1.as_ptr(), ptr);

            std::slice::from_raw_parts_mut(ptr, len)
                .iter()
                .map(|&v| Value::from_raw(NonNull::new_unchecked(v)))
                .collect()
        }
    }

    pub unsafe fn get_nth_param(&self, n: usize) -> Value {
        Value::from_raw(NonNull::new_unchecked(LLVMGetParam(
            self.1.as_ptr(),
            n as u32,
        )))
    }


    pub fn verify(&self, action: VerifierFailureAction) -> bool {
        unsafe {
            LLVMVerifyFunction(self.as_raw().as_ptr(), transmute(action)) != 0
        }
    }

    pub fn view_cfg(&self) {
        unsafe { LLVMViewFunctionCFG(self.as_raw().as_ptr()); }
    }

    pub fn view_cfg_only(&self) {
        unsafe { LLVMViewFunctionCFGOnly(self.as_raw().as_ptr()); }
    }

    pub fn get_subprogram(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetSubprogram(self.1.as_ptr())
            )
        })
    }

    pub fn set_subprogram(&self, subprogram: Metadata) {
        unsafe {
            LLVMSetSubprogram(self.1.as_ptr(), subprogram.as_raw().as_ptr())
        }
    }

    pub fn is_var_arg(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.2.as_raw().as_ptr()) != 0 }
    }

    pub fn get_return_type(&self) -> Type {
        Type::from_raw(self.2.as_raw())
    }

    // TODO: i'm not sure if these 2 following functions will behave correctly, especially because the 3rd argument of the Fun::from_raw call
    pub fn get_next_function(&self) -> Fun {
        Fun::from_raw(self.0, unsafe {
            NonNull::new_unchecked(
                LLVMGetNextFunction(self.1.as_ptr())
            )
        }, self.2)
    }

    pub fn get_previous_function(&self) -> Fun {
        Fun::from_raw(self.0, unsafe {
            NonNull::new_unchecked(
                LLVMGetPreviousFunction(self.1.as_ptr())
            )
        }, self.2)
    }

    pub fn get_basic_blocks(&self) -> Vec<BasicBlock> {
        unsafe {
            let len = self.get_arity();
            let ptr: *mut *mut LLVMBasicBlock =
                alloc(Layout::array::<*mut LLVMBasicBlock>(len).unwrap()) as _;
            LLVMGetBasicBlocks(self.1.as_ptr(), ptr);

            std::slice::from_raw_parts_mut(ptr, len)
                .iter()
                .map(|&basic_block| BasicBlock::from_raw(NonNull::new_unchecked(basic_block)))
                .collect()
        }
    }

    pub fn get_first_basic_block(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetFirstBasicBlock(self.1.as_ptr())
            )
        })
    }

    pub fn get_last_basic_block(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetLastBasicBlock(self.1.as_ptr())
            )
        })
    }

    pub fn get_entry_basic_block(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetEntryBasicBlock(self.1.as_ptr())
            )
        })
    }

    // TODO: is this okay (considering Drop trait)
    pub fn delete_function(&self) {
        unsafe { LLVMDeleteFunction(self.1.as_ptr()); }
    }

    pub fn has_personality_fn(&self) -> bool {
        unsafe { LLVMHasPersonalityFn(self.1.as_ptr()) != 0 }
    }

    pub fn get_personality_fn(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetPersonalityFn(self.1.as_ptr())
            )
        })
    }

    pub fn set_personality_fn(&self, personality_fn: Value) {
        unsafe { LLVMSetPersonalityFn(self.1.as_ptr(), personality_fn.as_raw().as_ptr()); }
    }

    pub fn get_intrinsic_id(&self) -> u32 {
        unsafe { LLVMGetIntrinsicID(self.1.as_ptr()) }
    }

    pub fn get_call_conv(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.1.as_ptr()) }
    }

    pub fn set_call_conv(&self, call_conv: u32) {
        unsafe { LLVMSetFunctionCallConv(self.1.as_ptr(), call_conv); }
    }

    pub fn get_gc(&self) -> &str {
        unsafe { from_c_string(LLVMGetGC(self.1.as_ptr())) }
    }

    pub fn set_gc(&self, name: &str) {
        unsafe { LLVMSetGC(self.1.as_ptr(), to_c_string(Some(name)).as_ptr()); }
    }

    pub fn count_basic_blocks(&self) -> u32 {
        unsafe { LLVMCountBasicBlocks(self.1.as_ptr()) }
    }

    // TODO: pub fn SetParamAlignment(Arg: LLVMValueRef, Align: u32);
    // TODO: pub fn GetParamParent(Inst: LLVMValueRef) -> LLVMValueRef;

    // TODO: pub fn LLVMGetNextGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;
    // TODO: pub fn LLVMGetPreviousGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;
    // TODO: pub fn LLVMGetGlobalIFuncResolver(IFunc: LLVMValueRef) -> LLVMValueRef;
    // TODO: pub fn LLVMSetGlobalIFuncResolver(IFunc: LLVMValueRef, Resolver: LLVMValueRef);
    // TODO: pub fn LLVMEraseGlobalIFunc(IFunc: LLVMValueRef);
    // TODO: pub fn LLVMRemoveGlobalIFunc(IFunc: LLVMValueRef);
}
