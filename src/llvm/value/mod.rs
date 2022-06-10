use std::mem::transmute;
use crate::llvm::sys::core::LLVMTypeOf;
use crate::llvm::sys::LLVMValue;
use crate::llvm::types::Type;
use std::ptr::NonNull;
use llvm_sys::comdat::{LLVMGetComdat, LLVMGetComdatSelectionKind, LLVMSetComdat, LLVMSetComdatSelectionKind};
use llvm_sys::debuginfo::{LLVMInstructionGetDebugLoc, LLVMInstructionSetDebugLoc};
use llvm_sys::LLVMComdat;
use crate::llvm::debug::Metadata;

#[derive(Debug, Copy, Clone)]
pub struct Value(NonNull<LLVMValue>);

impl Value {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMValue>) -> Value {
        Value(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMValue> {
        self.0
    }

    pub fn get_type(&self) -> Type {
        Type::from_raw(unsafe { NonNull::new_unchecked(LLVMTypeOf(self.0.as_ptr())) })
    }

    pub fn get_comdat(&self) -> Comdat {
        Comdat::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetComdat(self.0.as_ptr())
            )
        })
    }

    pub fn set_comdat(&self, comdat: Comdat) {
        unsafe {
            LLVMSetComdat(self.0.as_ptr(), comdat.0.as_ptr());
        }
    }

    pub fn instruction_get_debug_loc(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMInstructionGetDebugLoc(self.0.as_ptr())
            )
        })
    }

    pub fn instruction_set_debug_loc(&self, loc: Metadata) {
        unsafe {
            LLVMInstructionSetDebugLoc(self.0.as_ptr(), loc.as_raw().as_ptr());
        }
    }

    pub fn LLVMGetDebugLocDirectory(
        Val: LLVMValueRef,
        Length: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;
    pub fn LLVMGetDebugLocFilename(
        Val: LLVMValueRef,
        Length: *mut ::libc::c_uint,
    ) -> *const ::libc::c_char;
    pub fn LLVMGetDebugLocLine(Val: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetDebugLocColumn(Val: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetNextFunction(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousFunction(Fn: LLVMValueRef) -> LLVMValueRef;

    pub fn LLVMIsAArgument(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABasicBlock(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInlineAsm(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUser(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstant(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABlockAddress(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantAggregateZero(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantArray(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataSequential(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataArray(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataVector(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantExpr(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantFP(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantInt(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantPointerNull(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantStruct(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantTokenNone(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantVector(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalAlias(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalIFunc(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalObject(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFunction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalVariable(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUndefValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPoisonValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnaryOperator(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABinaryOperator(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACallInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntrinsicInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgInfoIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgVariableIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgDeclareInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgLabelInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemCpyInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemMoveInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemSetInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFCmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAICmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGetElementPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALandingPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPHINode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASelectInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAShuffleVectorInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAStoreInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABranchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIndirectBrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInvokeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASwitchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnreachableInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAResumeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACleanupReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACatchReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACatchSwitchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACallBrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFuncletPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACatchPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACleanupPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnaryInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAllocaInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAddrSpaceCastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABitCastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToSIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToUIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPTruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntToPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPtrToIntInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAZExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALoadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAVAArgInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFreezeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAtomicCmpXchgInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAtomicRMWInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFenceInst(Val: LLVMValueRef) -> LLVMValueRef;

    pub fn LLVMHasMetadata(Val: LLVMValueRef) -> ::libc::c_int;
    pub fn LLVMGetMetadata(Val: LLVMValueRef, KindID: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMSetMetadata(Val: LLVMValueRef, KindID: ::libc::c_uint, Node: LLVMValueRef);
    pub fn LLVMInstructionGetAllMetadataOtherThanDebugLoc(
        Instr: LLVMValueRef,
        NumEntries: *mut ::libc::size_t,
    ) -> *mut LLVMValueMetadataEntry;
    pub fn LLVMGetInstructionParent(Inst: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetNextInstruction(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousInstruction(Inst: LLVMValueRef) -> LLVMValueRef;
    /// Remove the given instruction from its containing building block but
    /// kept alive.
    pub fn LLVMInstructionRemoveFromParent(Inst: LLVMValueRef);
    /// Remove the given instruction from its containing building block and
    /// delete it.
    pub fn LLVMInstructionEraseFromParent(Inst: LLVMValueRef);
    pub fn LLVMGetInstructionOpcode(Inst: LLVMValueRef) -> LLVMOpcode;
    pub fn LLVMGetICmpPredicate(Inst: LLVMValueRef) -> LLVMIntPredicate;
    pub fn LLVMGetFCmpPredicate(Inst: LLVMValueRef) -> LLVMRealPredicate;
    pub fn LLVMInstructionClone(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATerminatorInst(Inst: LLVMValueRef) -> LLVMValueRef;

    // Instructions->Call Sites and Invocations
// Obtain the argument count for a call instruction.
//
// The provided value should be either a CallInst, InvokeInst or FuncletPadInst.
    pub fn LLVMGetNumArgOperands(Instr: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetInstructionCallConv(Instr: LLVMValueRef, CC: ::libc::c_uint);
    pub fn LLVMGetInstructionCallConv(Instr: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetInstrParamAlignment(
        Instr: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        Align: ::libc::c_uint,
    );
    pub fn LLVMAddCallSiteAttribute(C: LLVMValueRef, Idx: LLVMAttributeIndex, A: LLVMAttributeRef);
    pub fn LLVMGetCallSiteAttributeCount(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
    ) -> ::libc::c_uint;
    pub fn LLVMGetCallSiteAttributes(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        Attrs: *mut LLVMAttributeRef,
    );
    pub fn LLVMGetCallSiteEnumAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMGetCallSiteStringAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMRemoveCallSiteEnumAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    );
    pub fn LLVMRemoveCallSiteStringAttribute(
        C: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    );
    pub fn LLVMGetCalledFunctionType(C: LLVMValueRef) -> LLVMTypeRef;
    /// Get a pointer to the function invoked by this instruction.
    ///
    /// The provided value should be a CallInst or InvokeInst.
    pub fn LLVMGetCalledValue(Instr: LLVMValueRef) -> LLVMValueRef;
    /// Get whether a call instruction is a tail call.
    pub fn LLVMIsTailCall(CallInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetTailCall(CallInst: LLVMValueRef, IsTailCall: LLVMBool);
    /// Return the normal destination basic block of an invoke instruction.
    pub fn LLVMGetNormalDest(InvokeInst: LLVMValueRef) -> LLVMBasicBlockRef;
    /// Return the unwind destination basic block.
    pub fn LLVMGetUnwindDest(InvokeInst: LLVMValueRef) -> LLVMBasicBlockRef;
    /// Set the normal destination basic block.
    pub fn LLVMSetNormalDest(InvokeInst: LLVMValueRef, B: LLVMBasicBlockRef);
    /// Set the unwind destination basic block.
    pub fn LLVMSetUnwindDest(InvokeInst: LLVMValueRef, B: LLVMBasicBlockRef);

    // Instructions->Terminators
    pub fn LLVMGetNumSuccessors(Term: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetSuccessor(Term: LLVMValueRef, i: ::libc::c_uint) -> LLVMBasicBlockRef;
    pub fn LLVMSetSuccessor(Term: LLVMValueRef, i: ::libc::c_uint, block: LLVMBasicBlockRef);
    pub fn LLVMIsConditional(Branch: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetCondition(Branch: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetCondition(Branch: LLVMValueRef, Cond: LLVMValueRef);
    pub fn LLVMGetSwitchDefaultDest(SwitchInstr: LLVMValueRef) -> LLVMBasicBlockRef;

    // Instructions->Allocas
// Obtain the type being allocated by an alloca instruction.
    pub fn LLVMGetAllocatedType(Alloca: LLVMValueRef) -> LLVMTypeRef;

    pub fn LLVMIsInBounds(GEP: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetIsInBounds(GEP: LLVMValueRef, InBounds: LLVMBool);
    pub fn LLVMGetGEPSourceElementType(GEP: LLVMValueRef) -> LLVMTypeRef;
    pub fn LLVMAddIncoming(
        PhiNode: LLVMValueRef,
        IncomingValues: *mut LLVMValueRef,
        IncomingBlocks: *mut LLVMBasicBlockRef,
        Count: ::libc::c_uint,
    );
    pub fn LLVMCountIncoming(PhiNode: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetIncomingValue(PhiNode: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetIncomingBlock(PhiNode: LLVMValueRef, Index: ::libc::c_uint) -> LLVMBasicBlockRef;
    pub fn LLVMGetNumIndices(Inst: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetIndices(Inst: LLVMValueRef) -> *const ::libc::c_uint;

    pub fn LLVMAddCase(Switch: LLVMValueRef, OnVal: LLVMValueRef, Dest: LLVMBasicBlockRef);
    pub fn LLVMAddDestination(IndirectBr: LLVMValueRef, Dest: LLVMBasicBlockRef);
    pub fn LLVMGetNumClauses(LandingPad: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetClause(LandingPad: LLVMValueRef, Idx: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMAddClause(LandingPad: LLVMValueRef, ClauseVal: LLVMValueRef);
    pub fn LLVMIsCleanup(LandingPad: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetCleanup(LandingPad: LLVMValueRef, Val: LLVMBool);
    pub fn LLVMAddHandler(CatchSwitch: LLVMValueRef, Dest: LLVMBasicBlockRef);
    pub fn LLVMGetNumHandlers(CatchSwitch: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetHandlers(CatchSwitch: LLVMValueRef, Handlers: *mut LLVMBasicBlockRef);
    pub fn LLVMGetArgOperand(Funclet: LLVMValueRef, i: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMSetArgOperand(Funclet: LLVMValueRef, i: ::libc::c_uint, value: LLVMValueRef);
    pub fn LLVMGetParentCatchSwitch(CatchPad: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetParentCatchSwitch(CatchPad: LLVMValueRef, CatchSwitch: LLVMValueRef);

    pub fn LLVMGetNextGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousGlobalIFunc(IFunc: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetGlobalIFuncResolver(IFunc: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetGlobalIFuncResolver(IFunc: LLVMValueRef, Resolver: LLVMValueRef);
    pub fn LLVMEraseGlobalIFunc(IFunc: LLVMValueRef);
    pub fn LLVMRemoveGlobalIFunc(IFunc: LLVMValueRef);
    pub fn LLVMMDStringInContext2(
        C: LLVMContextRef,
        Str: *const ::libc::c_char,
        SLen: ::libc::size_t,
    ) -> LLVMMetadataRef;
    pub fn LLVMMDNodeInContext2(
        C: LLVMContextRef,
        MDs: *mut LLVMMetadataRef,
        Count: ::libc::size_t,
    ) -> LLVMMetadataRef;

    pub fn is_basic_block(Val: LLVMValueRef) -> LLVMBool;


    /// Obtain Metadata as a Value.
    pub fn LLVMMetadataAsValue(C: LLVMContextRef, MD: LLVMMetadataRef) -> LLVMValueRef;
    /// Obtain a Value as Metadata.
    pub fn LLVMValueAsMetadata(Val: LLVMValueRef) -> LLVMMetadataRef;
    /// Obtain the underlying string from a MDString value.
    ///
    /// `Len` is written to contain the length of the returned string.
    pub fn LLVMGetMDString(V: LLVMValueRef, Len: *mut ::libc::c_uint) -> *const ::libc::c_char;
    pub fn LLVMGetMDNodeNumOperands(V: LLVMValueRef) -> ::libc::c_uint;
        pub fn LLVMGetMDNodeOperands(V: LLVMValueRef, Dest: *mut LLVMValueRef);

    pub fn LLVMGetNumMaskElements(ShuffleVectorInst: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetUndefMaskElem() -> ::libc::c_int;
    pub fn LLVMGetMaskValue(ShuffleVectorInst: LLVMValueRef, Elt: ::libc::c_uint) -> ::libc::c_int;
    pub fn LLVMIsAtomicSingleThread(AtomicInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetAtomicSingleThread(AtomicInst: LLVMValueRef, SingleThread: LLVMBool);
    pub fn LLVMGetCmpXchgSuccessOrdering(CmpXchgInst: LLVMValueRef) -> LLVMAtomicOrdering;
    pub fn LLVMSetCmpXchgSuccessOrdering(CmpXchgInst: LLVMValueRef, Ordering: LLVMAtomicOrdering);
    pub fn LLVMGetCmpXchgFailureOrdering(CmpXchgInst: LLVMValueRef) -> LLVMAtomicOrdering;
    pub fn LLVMSetCmpXchgFailureOrdering(CmpXchgInst: LLVMValueRef, Ordering: LLVMAtomicOrdering);
    pub fn LLVMGetElementAsConstant(C: LLVMValueRef, idx: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstVector(
        ScalarConstantVals: *mut LLVMValueRef,
        Size: ::libc::c_uint,
    ) -> LLVMValueRef;

    pub fn LLVMGetConstOpcode(ConstantVal: LLVMValueRef) -> LLVMOpcode;
    pub fn LLVMAlignOf(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMSizeOf(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNot(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstUDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstExactUDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef)
                              -> LLVMValueRef;
    pub fn LLVMConstSDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstExactSDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef)
                              -> LLVMValueRef;
    pub fn LLVMConstFDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstURem(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstSRem(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFRem(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAnd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstOr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstXor(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstICmp(
        Predicate: LLVMIntPredicate,
        LHSConstant: LLVMValueRef,
        RHSConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstFCmp(
        Predicate: LLVMRealPredicate,
        LHSConstant: LLVMValueRef,
        RHSConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstShl(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstLShr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAShr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstGEP2(
        Ty: LLVMTypeRef,
        ConstantVal: LLVMValueRef,
        ConstantIndices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstInBoundsGEP2(
        Ty: LLVMTypeRef,
        ConstantVal: LLVMValueRef,
        ConstantIndices: *mut LLVMValueRef,
        NumIndices: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstZExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstUIToFP(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSIToFP(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPToUI(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPToSI(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstPtrToInt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstIntToPtr(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstAddrSpaceCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstZExtOrBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSExtOrBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstTruncOrBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstPointerCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstIntCast(
        ConstantVal: LLVMValueRef,
        ToType: LLVMTypeRef,
        isSigned: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstFPCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSelect(
        ConstantCondition: LLVMValueRef,
        ConstantIfTrue: LLVMValueRef,
        ConstantIfFalse: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstExtractElement(
        VectorConstant: LLVMValueRef,
        IndexConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstInsertElement(
        VectorConstant: LLVMValueRef,
        ElementValueConstant: LLVMValueRef,
        IndexConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstShuffleVector(
        VectorAConstant: LLVMValueRef,
        VectorBConstant: LLVMValueRef,
        MaskConstant: LLVMValueRef,
    ) -> LLVMValueRef;
    pub fn LLVMConstExtractValue(
        AggConstant: LLVMValueRef,
        IdxList: *mut ::libc::c_uint,
        NumIdx: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstInsertValue(
        AggConstant: LLVMValueRef,
        ElementValueConstant: LLVMValueRef,
        IdxList: *mut ::libc::c_uint,
        NumIdx: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMBlockAddress(F: LLVMValueRef, BB: LLVMBasicBlockRef) -> LLVMValueRef;

    // Core->Values->Constants->Global Values
    pub fn LLVMGetGlobalParent(Global: LLVMValueRef) -> LLVMModuleRef;
    pub fn LLVMIsDeclaration(Global: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetLinkage(Global: LLVMValueRef) -> LLVMLinkage;
    pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: LLVMLinkage);
    pub fn LLVMGetSection(Global: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMSetSection(Global: LLVMValueRef, Section: *const ::libc::c_char);
    pub fn LLVMGetVisibility(Global: LLVMValueRef) -> LLVMVisibility;
    pub fn LLVMSetVisibility(Global: LLVMValueRef, Viz: LLVMVisibility);
    pub fn LLVMGetDLLStorageClass(Global: LLVMValueRef) -> LLVMDLLStorageClass;
    pub fn LLVMSetDLLStorageClass(Global: LLVMValueRef, Class: LLVMDLLStorageClass);

    pub fn LLVMGetUnnamedAddress(Global: LLVMValueRef) -> LLVMUnnamedAddr;
    pub fn LLVMSetUnnamedAddress(Global: LLVMValueRef, UnnamedAddr: LLVMUnnamedAddr);
    pub fn LLVMGlobalGetValueType(Global: LLVMValueRef) -> LLVMTypeRef;

    pub fn LLVMGetAlignment(V: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetAlignment(V: LLVMValueRef, Bytes: ::libc::c_uint);

    pub fn LLVMGlobalSetMetadata(Global: LLVMValueRef, Kind: ::libc::c_uint, MD: LLVMMetadataRef);
    pub fn LLVMGlobalEraseMetadata(Global: LLVMValueRef, Kind: ::libc::c_uint);
    pub fn LLVMGlobalClearMetadata(Global: LLVMValueRef);
    pub fn LLVMGetNextGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMDeleteGlobal(GlobalVar: LLVMValueRef);
    pub fn LLVMGetInitializer(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetInitializer(GlobalVar: LLVMValueRef, ConstantVal: LLVMValueRef);
    pub fn LLVMIsThreadLocal(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetThreadLocal(GlobalVar: LLVMValueRef, IsThreadLocal: LLVMBool);
    pub fn LLVMIsGlobalConstant(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetGlobalConstant(GlobalVar: LLVMValueRef, IsConstant: LLVMBool);
    pub fn LLVMGetThreadLocalMode(GlobalVar: LLVMValueRef) -> LLVMThreadLocalMode;
    pub fn LLVMSetThreadLocalMode(GlobalVar: LLVMValueRef, Mode: LLVMThreadLocalMode);
    pub fn LLVMIsExternallyInitialized(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetExternallyInitialized(GlobalVar: LLVMValueRef, IsExtInit: LLVMBool);


    pub fn LLVMGetValueKind(Val: LLVMValueRef) -> LLVMValueKind;
    pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
    pub fn LLVMGetValueName2(
        Val: LLVMValueRef,
        Length: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMSetValueName2(
        Val: LLVMValueRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    );
    pub fn LLVMDumpValue(Val: LLVMValueRef);
    pub fn LLVMPrintValueToString(Val: LLVMValueRef) -> *mut ::libc::c_char;
    pub fn LLVMReplaceAllUsesWith(OldVal: LLVMValueRef, NewVal: LLVMValueRef);
    pub fn LLVMIsConstant(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMIsUndef(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMIsPoison(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMIsAMDNode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMDString(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetOperand(Val: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetOperandUse(Val: LLVMValueRef, Index: ::libc::c_uint) -> LLVMUseRef;
    pub fn LLVMSetOperand(User: LLVMValueRef, Index: ::libc::c_uint, Val: LLVMValueRef);
    pub fn LLVMGetNumOperands(Val: LLVMValueRef) -> ::libc::c_int;


    // ..->Function Values
    pub fn LLVMDeleteFunction(Fn: LLVMValueRef);
    pub fn LLVMHasPersonalityFn(Fn: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetPersonalityFn(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetPersonalityFn(Fn: LLVMValueRef, PersonalityFn: LLVMValueRef);
    pub fn LLVMLookupIntrinsicID(
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> ::libc::c_uint;
    pub fn LLVMGetIntrinsicID(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetIntrinsicDeclaration(
        Mod: LLVMModuleRef,
        ID: ::libc::c_uint,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::size_t,
    ) -> LLVMValueRef;
    pub fn LLVMIntrinsicGetType(
        Ctx: LLVMContextRef,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::size_t,
    ) -> LLVMTypeRef;
    pub fn LLVMIntrinsicGetName(
        ID: ::libc::c_uint,
        NameLength: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMIntrinsicCopyOverloadedName2(
        Mod: LLVMModuleRef,
        ID: ::libc::c_uint,
        ParamTypes: *mut LLVMTypeRef,
        ParamCount: ::libc::size_t,
        NameLength: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMIntrinsicIsOverloaded(ID: ::libc::c_uint) -> LLVMBool;
    pub fn LLVMGetFunctionCallConv(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetFunctionCallConv(Fn: LLVMValueRef, CC: ::libc::c_uint);
    pub fn LLVMGetGC(Fn: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMSetGC(Fn: LLVMValueRef, Name: *const ::libc::c_char);
    pub fn LLVMAddAttributeAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex, A: LLVMAttributeRef);
    pub fn LLVMGetAttributeCountAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex) -> ::libc::c_uint;
    pub fn LLVMGetAttributesAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        Attrs: *mut LLVMAttributeRef,
    );
    pub fn LLVMGetEnumAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMGetStringAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    ) -> LLVMAttributeRef;
    pub fn LLVMRemoveEnumAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        KindID: ::libc::c_uint,
    );
    pub fn LLVMRemoveStringAttributeAtIndex(
        F: LLVMValueRef,
        Idx: LLVMAttributeIndex,
        K: *const ::libc::c_char,
        KLen: ::libc::c_uint,
    );
    pub fn LLVMAddTargetDependentFunctionAttr(
        Fn: LLVMValueRef,
        A: *const ::libc::c_char,
        V: *const ::libc::c_char,
    );

    // ..->Function Values->Function Parameters
    pub fn LLVMCountParams(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetParams(Fn: LLVMValueRef, Params: *mut LLVMValueRef);
    pub fn LLVMGetParam(Fn: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetParamParent(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetFirstParam(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetLastParam(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetNextParam(Arg: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousParam(Arg: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetParamAlignment(Arg: LLVMValueRef, Align: ::libc::c_uint);


    pub fn LLVMIsConstantString(c: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetAsString(C: LLVMValueRef, Length: *mut ::libc::size_t) -> *const ::libc::c_char;


    pub fn LLVMConstIntGetZExtValue(ConstantVal: LLVMValueRef) -> ::libc::c_ulonglong;
    pub fn LLVMConstIntGetSExtValue(ConstantVal: LLVMValueRef) -> ::libc::c_longlong;
    pub fn LLVMConstRealGetDouble(
        ConstantVal: LLVMValueRef,
        losesInfo: *mut LLVMBool,
    ) -> ::libc::c_double;


    pub fn LLVMConstStringInContext(
        C: LLVMContextRef,
        Str: *const ::libc::c_char,
        Length: ::libc::c_uint,
        DontNullTerminate: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstString(
        Str: *const ::libc::c_char,
        Length: ::libc::c_uint,
        DontNullTerminate: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstStructInContext(
        C: LLVMContextRef,
        ConstantVals: *mut LLVMValueRef,
        Count: ::libc::c_uint,
        Packed: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstStruct(
        ConstantVals: *mut LLVMValueRef,
        Count: ::libc::c_uint,
        Packed: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstArray(
        ElementTy: LLVMTypeRef,
        ConstantVals: *mut LLVMValueRef,
        Length: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMConstNamedStruct(
        StructTy: LLVMTypeRef,
        ConstantVals: *mut LLVMValueRef,
        Count: ::libc::c_uint,
    ) -> LLVMValueRef;


    pub fn LLVMGlobalCopyAllMetadata(
        Value: LLVMValueRef,
        NumEntries: *mut ::libc::size_t,
    ) -> *mut LLVMValueMetadataEntry;
}

#[derive(Debug, Copy, Clone)]
pub struct Comdat(NonNull<LLVMComdat>);

impl Comdat {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMComdat>) -> Comdat {
        Comdat(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMComdat> {
        self.0
    }

    pub fn get_comdat_selection_kind(&self) -> ComdatSelectionKind {
        unsafe {
            transmute(LLVMGetComdatSelectionKind(self.0.as_ptr()))
        }
    }

    pub fn set_comdat_selection_kind(&self, comdat_selection_kind: ComdatSelectionKind) {
        unsafe {
            LLVMSetComdatSelectionKind(self.0.as_ptr(), transmute(comdat_selection_kind));
        }
    }
}

#[repr(C)]
pub enum ComdatSelectionKind {
    AnyComdatSelectionKind,
    ExactMatchComdatSelectionKind,
    LargestComdatSelectionKind,
    NoDuplicatesComdatSelectionKind,
    SameSizeComdatSelectionKind,
}


pub fn LLVMDisposeValueMetadataEntries(Entries: *mut LLVMValueMetadataEntry);
pub fn LLVMValueMetadataEntriesGetKind(
    Entries: *mut LLVMValueMetadataEntry,
    Index: ::libc::c_uint,
) -> ::libc::c_uint;
pub fn LLVMValueMetadataEntriesGetMetadata(
    Entries: *mut LLVMValueMetadataEntry,
    Index: ::libc::c_uint,
) -> LLVMMetadataRef;

pub fn LLVMGetFirstUse(Val: LLVMValueRef) -> LLVMUseRef;
pub fn LLVMGetNextUse(U: LLVMUseRef) -> LLVMUseRef;
pub fn LLVMGetUser(U: LLVMUseRef) -> LLVMValueRef;
pub fn LLVMGetUsedValue(U: LLVMUseRef) -> LLVMValueRef;
pub fn LLVMDisposeModuleProvider(M: LLVMModuleProviderRef);
pub fn LLVMGetGlobalPassRegistry() -> LLVMPassRegistryRef;