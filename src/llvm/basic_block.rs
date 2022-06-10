use crate::llvm::sys::LLVMBasicBlock;
use std::ptr::NonNull;

#[derive(Debug, Copy, Clone)]
pub struct BasicBlock(NonNull<LLVMBasicBlock>);

impl BasicBlock {
    pub fn from_raw(raw: NonNull<LLVMBasicBlock>) -> BasicBlock {
        BasicBlock(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMBasicBlock> {
        self.0
    }

    pub fn as_value(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn ValueAsBasicBlock(Val: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn GetBasicBlockName(BB: LLVMBasicBlockRef) -> *const ::libc::c_char;
    pub fn GetBasicBlockParent(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn GetBasicBlockTerminator(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn CountBasicBlocks(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn GetBasicBlocks(Fn: LLVMValueRef, BasicBlocks: *mut LLVMBasicBlockRef);
    pub fn GetFirstBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn GetLastBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn GetNextBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;
    pub fn GetPreviousBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;
    pub fn GetEntryBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    /// Insert the given basic block after the insertion point of the given builder.
    pub fn LLVMInsertExistingBasicBlockAfterInsertBlock(
        Builder: LLVMBuilderRef,
        BB: LLVMBasicBlockRef,
    );
    /// Append the given basic block to the basic block list of the given function.
    pub fn LLVMAppendExistingBasicBlock(Fn: LLVMValueRef, BB: LLVMBasicBlockRef);
    pub fn LLVMCreateBasicBlockInContext(
        C: LLVMContextRef,
        Name: *const ::libc::c_char,
    ) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlockInContext(
        C: LLVMContextRef,
        Fn: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlock(Fn: LLVMValueRef, Name: *const ::libc::c_char)
                                -> LLVMBasicBlockRef;
    pub fn LLVMInsertBasicBlockInContext(
        C: LLVMContextRef,
        BB: LLVMBasicBlockRef,
        Name: *const ::libc::c_char,
    ) -> LLVMBasicBlockRef;
    pub fn LLVMInsertBasicBlock(
        InsertBeforeBB: LLVMBasicBlockRef,
        Name: *const ::libc::c_char,
    ) -> LLVMBasicBlockRef;
    pub fn LLVMDeleteBasicBlock(BB: LLVMBasicBlockRef);
    pub fn LLVMRemoveBasicBlockFromParent(BB: LLVMBasicBlockRef);
    pub fn LLVMMoveBasicBlockBefore(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef);
    pub fn LLVMMoveBasicBlockAfter(BB: LLVMBasicBlockRef, MovePos: LLVMBasicBlockRef);
    pub fn LLVMGetFirstInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMGetLastInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
}
