use std::mem::{MaybeUninit, transmute};
use crate::llvm::sys::core::LLVMTypeOf;
use crate::llvm::sys::LLVMValue;
use crate::llvm::types::Type;
use std::ptr::NonNull;
use std::slice;
use llvm_sys::comdat::{LLVMGetComdat, LLVMGetComdatSelectionKind, LLVMSetComdat, LLVMSetComdatSelectionKind};
use llvm_sys::debuginfo::{LLVMInstructionGetDebugLoc, LLVMInstructionSetDebugLoc};
use llvm_sys::*;
use llvm_sys::core::*;
use crate::llvm::*;
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::context::{Attribute, Context};
use crate::llvm::debug::Metadata;
use crate::llvm::module::Module;
use crate::llvm::opcode::Opcode;

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

    pub fn get_volatile(&self) -> bool {
        unsafe { LLVMGetVolatile(self.0.as_ptr()) != 0 }
    }
    
    pub fn set_volatile(&self, is_volatile: bool) {
        unsafe { LLVMSetVolatile(self.0.as_ptr(), is_volatile as i32); }
    }
    
    pub fn get_weak(&self) -> bool {
        unsafe { LLVMGetWeak(self.0.as_ptr()) != 0 }
    }
    
    pub fn set_weak(&self, is_weak: bool) {
        unsafe { LLVMSetWeak(self.0.as_ptr(), is_weak as i32); }
    }
    
    pub fn get_ordering(&self) -> AtomicOrdering {
        unsafe {
            transmute(LLVMGetOrdering(self.0.as_ptr()))
        }
    }
    
    pub fn set_ordering(&self, ordering: AtomicOrdering) {
        unsafe {
            LLVMSetOrdering(self.0.as_ptr(), transmute(ordering));
        }
    }
    
    pub fn get_atomic_rmw_bin_op(&self) -> AtomicRMWBinOp {
        unsafe {
            transmute(LLVMGetAtomicRMWBinOp(self.0.as_ptr()))
        }
    }
    
    pub fn set_atomic_rmw_bin_op(&self, bin_op: AtomicRMWBinOp) {
        unsafe {
            LLVMSetAtomicRMWBinOp(self.0.as_ptr(), transmute(bin_op));
        }
    }
    
    pub fn get_debug_loc_directory(&self) -> String {
        let mut len = MaybeUninit::uninit();
        let chars = unsafe { LLVMGetDebugLocDirectory(self.0.as_ptr(), len.as_mut_ptr()) };
        
        String::from_utf8(unsafe { 
            transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init() as usize))
        }.to_vec()).unwrap()
    }
    
    pub fn get_debug_loc_filename(&self) -> String {
        let mut len = MaybeUninit::uninit();
        let chars = unsafe { LLVMGetDebugLocDirectory(self.0.as_ptr(), len.as_mut_ptr()) };
        
        String::from_utf8(unsafe {
            transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init() as usize))
        }.to_vec()).unwrap()
    }
    
    pub fn get_debug_loc_line(&self) -> u32 {
        unsafe { LLVMGetDebugLocLine(self.0.as_ptr()) }
    }
    
    pub fn get_debug_loc_column(&self) -> u32 {
        unsafe { LLVMGetDebugLocColumn(self.0.as_ptr()) }
    }

    pub fn is_an_argument(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAArgument(self.0.as_ptr()))
        })
    }

    pub fn is_a_basic_block(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsABasicBlock(self.0.as_ptr()))
        })
    }

    pub fn is_an_inline_asm(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAInlineAsm(self.0.as_ptr()))
        })
    }

    pub fn is_a_user(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAUser(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstant(self.0.as_ptr()))
        })
    }

    pub fn is_a_block_address(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsABlockAddress(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_aggregate_zero(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantAggregateZero(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_array(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantArray(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_data_sequential(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantDataSequential(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_data_array(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantDataArray(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_data_vector(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantDataVector(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_expr(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantExpr(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_fp(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantFP(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_int(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantInt(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_pointer_null(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantPointerNull(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_struct(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantStruct(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_token_none(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantTokenNone(self.0.as_ptr()))
        })
    }

    pub fn is_a_constant_vector(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAConstantVector(self.0.as_ptr()))
        })
    }

    pub fn is_a_global_value(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAGlobalValue(self.0.as_ptr()))
        })
    }

    pub fn is_a_global_alias(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAGlobalAlias(self.0.as_ptr()))
        })
    }

    pub fn is_a_global_ifunc(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAGlobalIFunc(self.0.as_ptr()))
        })
    }

    pub fn is_a_global_object(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAGlobalObject(self.0.as_ptr()))
        })
    }

    pub fn is_a_function(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFunction(self.0.as_ptr()))
        })
    }

    pub fn is_a_global_variable(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAGlobalVariable(self.0.as_ptr()))
        })
    }

    pub fn is_an_undef_value(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAUndefValue(self.0.as_ptr()))
        })
    }

    pub fn is_a_poison_value(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAPoisonValue(self.0.as_ptr()))
        })
    }

    pub fn is_a_instruction(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAInstruction(self.0.as_ptr()))
        })
    }

    pub fn is_a_unary_operator(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAUnaryOperator(self.0.as_ptr()))
        })
    }

    pub fn is_a_binary_operator(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsABinaryOperator(self.0.as_ptr()))
        })
    }

    pub fn is_a_call_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACallInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_intrinsic_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAIntrinsicInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_dbg_info_intrinsic(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsADbgInfoIntrinsic(self.0.as_ptr()))
        })
    }

    pub fn is_a_dbg_variable_intrinsic(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsADbgVariableIntrinsic(self.0.as_ptr()))
        })
    }

    pub fn is_a_dbg_declare_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsADbgDeclareInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_dbg_label_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsADbgLabelInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_mem_intrinsic(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAMemIntrinsic(self.0.as_ptr()))
        })
    }

    pub fn is_a_mem_cpy_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAMemCpyInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_mem_move_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAMemMoveInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_mem_set_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAMemSetInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_cmp_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACmpInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_fcmp_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFCmpInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_icmp_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAICmpInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_extract_element_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAExtractElementInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_get_element_ptr_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAGetElementPtrInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_insert_element_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAInsertElementInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_insert_value_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAInsertValueInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_landing_pad_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsALandingPadInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_phinode(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAPHINode(self.0.as_ptr()))
        })
    }

    pub fn is_a_select_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsASelectInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_shuffle_vector_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAShuffleVectorInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_store_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAStoreInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_branch_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsABranchInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_indirect_br_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAIndirectBrInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_invoke_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAInvokeInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_return_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAReturnInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_switch_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsASwitchInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_unreachable_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAUnreachableInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_resume_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAResumeInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_cleanup_return_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACleanupReturnInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_catch_return_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACatchReturnInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_catch_switch_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACatchSwitchInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_call_br_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACallBrInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_funclet_pad_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFuncletPadInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_catch_pad_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACatchPadInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_cleanup_pad_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACleanupPadInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_unary_instruction(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAUnaryInstruction(self.0.as_ptr()))
        })
    }

    pub fn is_an_alloca_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAAllocaInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_cast_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsACastInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_addr_space_cast_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAAddrSpaceCastInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_bit_cast_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsABitCastInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_fpext_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFPExtInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_fpto_siinst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFPToSIInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_fpto_uiinst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFPToUIInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_fptrunc_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFPTruncInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_int_to_ptr_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAIntToPtrInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_ptr_to_int_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAPtrToIntInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_sext_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsASExtInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_sito_fpinst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsASIToFPInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_trunc_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsATruncInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_uito_fpinst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAUIToFPInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_zext_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAZExtInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_extract_value_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAExtractValueInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_load_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsALoadInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_va_arg_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAVAArgInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_freeze_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFreezeInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_atomic_cmp_xchg_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAAtomicCmpXchgInst(self.0.as_ptr()))
        })
    }

    pub fn is_an_atomic_rmwinst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAAtomicRMWInst(self.0.as_ptr()))
        })
    }

    pub fn is_a_fence_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsAFenceInst(self.0.as_ptr()))
        })
    }

    pub fn has_metadata(&self) -> i32 {
        unsafe {
            LLVMHasMetadata(self.0.as_ptr())
        }
    }

    pub fn get_metadata(&self, id: u32) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetMetadata(self.0.as_ptr(), id))
        })
    }

    pub fn set_metadata(&self, id: u32, node: Value) {
       unsafe {
           LLVMSetMetadata(self.0.as_ptr(), id, node.0.as_ptr())
       }
    }

    // pub fn InstructionGetAllMetadataOtherThanDebugLoc(&self, NumEntries: *mut ::libc::size_t) -> *mut LLVMValueMetadataEntry;

    pub fn get_instruction_parent(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetInstructionParent(self.0.as_ptr()))
        })
    }

    pub fn get_next_instruction(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetNextInstruction(self.0.as_ptr()))
        })
    }

    pub fn get_previous_instruction(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetPreviousInstruction(self.0.as_ptr()))
        })
    }

    pub fn instruction_remove_from_parent(&self) {
        unsafe { LLVMInstructionRemoveFromParent(self.0.as_ptr()); }
    }

    pub fn instruction_erase_from_parent(&self) {
        unsafe { LLVMInstructionEraseFromParent(self.0.as_ptr()); }
    }
    
    pub fn get_instruction_opcode(&self) -> Opcode {
        unsafe { transmute(LLVMGetInstructionOpcode(self.0.as_ptr())) }
    }
    
    pub fn get_icmp_predicate(&self) -> IntPredicate {
        unsafe { transmute(LLVMGetICmpPredicate(self.0.as_ptr())) }
    }
    
    pub fn get_fcmp_predicate(&self) -> FloatPredicate {
        unsafe { transmute(LLVMGetFCmpPredicate(self.0.as_ptr())) }
    }
    
    pub fn instruction_clone(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMInstructionClone(self.0.as_ptr()))
        })
    }
    
    pub fn is_a_terminator_inst(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMIsATerminatorInst(self.0.as_ptr()))
        })
    }

    pub fn get_num_arg_operands(&self) -> u32 {
        unsafe { LLVMGetNumArgOperands(self.0.as_ptr()) }
    }

    pub fn set_instruction_call_conv(&self, call_conv: u32) {
        unsafe {
            LLVMSetInstructionCallConv(self.0.as_ptr(), call_conv);
        }
    }

    pub fn get_instruction_call_conv(&self) -> u32 {
        unsafe { LLVMGetInstructionCallConv(self.0.as_ptr()) }
    }

    pub fn set_instr_param_alignment(&self, attribute_index: u32, align: u32) {
        unsafe { LLVMSetInstrParamAlignment(self.0.as_ptr(), attribute_index, align); }
    }

    pub fn add_call_site_attribute(&self, attribute_index: u32, attribute: Attribute) {
        unsafe { LLVMAddCallSiteAttribute(self.0.as_ptr(), attribute_index, attribute.as_raw().as_ptr()); }
    }

    pub fn get_call_site_attribute_count(&self, attribute_index: u32) -> u32 {
        unsafe { LLVMGetCallSiteAttributeCount(self.0.as_ptr(), attribute_index) }
    }

    /*pub fn get_call_site_attributes(&self, attribute_index: u32, attributes: Vec<Attribute>) {
        unsafe {
            LLVMGetCallSiteAttributes(
                self.0.as_ptr(),
                attribute_index,
                attributes
            )
            // TODO: LLVMGetCallSiteAttributes()
            todo!()
        }
    }*/

    pub fn get_call_site_enum_attribute(&self, attribute_index: u32, kind_id: u32) -> Attribute {
        Attribute::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetCallSiteEnumAttribute(self.0.as_ptr(), attribute_index, kind_id))
        })
    }

    pub fn get_call_site_string_attribute(&self, attribute_index: u32, k: &str) -> Attribute {
        Attribute::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetCallSiteStringAttribute(
                self.0.as_ptr(),
                attribute_index,
                to_c_string(Some(k)).as_ptr(),
                k.len() as u32
            ))
        })
    }

    pub fn remove_call_site_enum_attribute(&self, attribute_index: u32, kind_id: u32) {
        unsafe {
            LLVMRemoveCallSiteEnumAttribute(self.0.as_ptr(), attribute_index, kind_id)
        }
    }

    pub fn remove_call_site_string_attribute(&self, attribute_index: u32, k: &str) {
        unsafe {
            LLVMRemoveCallSiteStringAttribute(self.0.as_ptr(), attribute_index, to_c_string(Some(k)).as_ptr(), k.len() as u32)
        }
    }

    pub fn get_called_function_type(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetCalledFunctionType(self.0.as_ptr()))
        })
    }

    pub fn get_called_value(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetCalledValue(self.0.as_ptr()))
        })
    }

    pub fn is_tail_call(&self) -> bool {
        unsafe { LLVMIsTailCall(self.0.as_ptr()) != 0 }
    }

    pub fn set_tail_call(&self, is_tail_call: bool) {
        unsafe { LLVMSetTailCall(self.0.as_ptr(), is_tail_call as i32); }
    }

    pub fn get_normal_dest(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetNormalDest(self.0.as_ptr()))
        })
    }

    pub fn get_unwind_dest(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetUnwindDest(self.0.as_ptr()))
        })
    }

    pub fn set_normal_dest(&self, basic_block: BasicBlock) {
        unsafe { LLVMSetNormalDest(self.0.as_ptr(), basic_block.as_raw().as_ptr()); }
    }

    pub fn set_unwind_dest(&self, basic_block: BasicBlock) {
        unsafe { LLVMSetUnwindDest(self.0.as_ptr(), basic_block.as_raw().as_ptr()); }
    }

    pub fn get_num_successors(&self) -> u32 {
        unsafe { LLVMGetNumSuccessors(self.0.as_ptr()) }
    }

    pub fn get_successor(&self, i: u32) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetSuccessor(self.0.as_ptr(), i))
        })
    }

    pub fn set_successor(&self, i: u32, block: BasicBlock) {
        unsafe { LLVMSetSuccessor(self.0.as_ptr(), i, block.as_raw().as_ptr()) }
    }

    pub fn is_conditional(&self) -> bool {
        unsafe { LLVMIsConditional(self.0.as_ptr()) != 0 }
    }

    pub fn get_condition(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetCondition(self.0.as_ptr()))
        })
    }

    pub fn set_condition(&self, cond: Value) {
        unsafe {
            LLVMSetCondition(self.0.as_ptr(), cond.0.as_ptr())
        }
    }

    pub fn get_switch_default_dest(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetSwitchDefaultDest(self.0.as_ptr()))
        })
    }

    pub fn get_allocated_type(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetAllocatedType(self.0.as_ptr()))
        })
    }

    pub fn is_in_bounds(&self) -> bool {
        unsafe { LLVMIsInBounds(self.0.as_ptr()) != 0 }
    }

    pub fn set_is_in_bounds(&self, in_bounds: bool) {
        unsafe { LLVMSetIsInBounds(self.0.as_ptr(), in_bounds as i32); }
    }

    pub fn get_gep_source_element_type(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetGEPSourceElementType(self.0.as_ptr())
            )
        })
    }

    pub fn add_incoming(&self, incoming: Vec<(Value, BasicBlock)>) {
        let mut values = vec![];
        let mut basic_blocks = vec![];
        for (value, basic_block) in incoming {
            values.push(value.as_raw().as_ptr());
            basic_blocks.push(basic_block.as_raw().as_ptr());
        }

        unsafe {
            LLVMAddIncoming(self.0.as_ptr(), values.as_mut_ptr(),
                            basic_blocks.as_mut_ptr(), values.len() as u32)
        }
    }


    pub fn count_incoming(&self) -> u32 {
        unsafe { LLVMCountIncoming(self.0.as_ptr()) }
    }

    pub fn get_incoming_value(&self, index: u32) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetIncomingValue(self.0.as_ptr(), index))
        })
    }

    pub fn get_incoming_block(&self, index: u32) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetIncomingBlock(self.0.as_ptr(), index))
        })
    }

    pub fn get_num_indices(&self) -> u32 {
        unsafe {
            LLVMGetNumIndices(self.0.as_ptr())
        }
    }

    pub fn get_indices(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(
                LLVMGetIndices(self.0.as_ptr()), LLVMGetNumIndices(self.0.as_ptr()) as usize
            )
        }
    }

    pub fn add_case(&self, on_val: Value, dest: BasicBlock) {
        unsafe {
            LLVMAddCase(self.0.as_ptr(), on_val.0.as_ptr(), dest.as_raw().as_ptr())
        }
    }

    pub fn add_destination(&self, dest: BasicBlock) {
        unsafe {
            LLVMAddDestination(self.0.as_ptr(), dest.as_raw().as_ptr())
        }
    }

    pub fn get_num_clauses(&self) -> u32 {
        unsafe {
            LLVMGetNumClauses(self.0.as_ptr())
        }
    }

    pub fn get_clause(&self, index: u32) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetClause(self.0.as_ptr(), index))
        })
    }

    pub fn add_clause(&self, clause_val: Value) {
        unsafe {
            LLVMAddClause(self.0.as_ptr(), clause_val.0.as_ptr())
        }
    }

    pub fn is_cleanup(&self) -> bool {
        unsafe { LLVMIsCleanup(self.0.as_ptr()) != 0 }
    }

    pub fn set_cleanup(&self, val: bool) {
        unsafe {
            LLVMSetCleanup(self.0.as_ptr(), val as i32)
        }
    }

    pub fn add_handler(&self, dest: BasicBlock) {
        unsafe {
            LLVMAddHandler(self.0.as_ptr(), dest.as_raw().as_ptr())
        }
    }

    pub fn get_num_handlers(&self) -> u32 {
        unsafe {
            LLVMGetNumHandlers(self.0.as_ptr())
        }
    }

    pub fn get_handlers(&self, handlers: Vec<BasicBlock>) {
        unsafe {
            LLVMGetHandlers(self.0.as_ptr(), handlers.iter()
                .map(|handler| handler.as_raw().as_ptr())
                .collect::<Vec<_>>()
                .as_mut_ptr())
        }
    }

    pub fn get_arg_operand(&self, i: u32) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetArgOperand(self.0.as_ptr(), i))
        })
    }

    pub fn set_arg_operand(&self, i: u32, value: Value) {
        unsafe { LLVMSetArgOperand(self.0.as_ptr(), i, value.as_raw().as_ptr()) }
    }

    pub fn get_parent_catch_switch(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetParentCatchSwitch(self.0.as_ptr()))
        })
    }

    pub fn set_parent_catch_switch(&self, catch_switch: Value) {
        unsafe { LLVMSetParentCatchSwitch(self.0.as_ptr(), catch_switch.0.as_ptr()) }
    }

    pub fn is_basic_block(&self) -> bool {
        unsafe { LLVMValueIsBasicBlock(self.0.as_ptr()) != 0 }
    }

    pub fn as_metadata(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMValueAsMetadata(self.0.as_ptr())
            )
        })
    }

    pub fn get_metadata_string(&self) -> String {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let chars = LLVMGetMDString(self.0.as_ptr(), len.as_mut_ptr());

            String::from_utf8(
                transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init() as usize)).to_vec()
            ).unwrap()
        }
    }

    pub fn get_metadata_node_num_operands(&self) -> u32 {
        unsafe { LLVMGetMDNodeNumOperands(self.0.as_ptr()) }
    }

    pub fn get_metadata_node_operands(&self) -> Value {
        Value::from_raw(unsafe {
            let mut value = MaybeUninit::uninit();
            LLVMGetMDNodeOperands(self.0.as_ptr(), value.as_mut_ptr());

            NonNull::new_unchecked(value.assume_init())
        })
    }

    pub fn get_num_mask_elements(&self) -> u32 {
        unsafe { LLVMGetNumMaskElements(self.0.as_ptr()) }
    }

    pub fn get_undef_mask_elem() -> i32 {
        unsafe { LLVMGetUndefMaskElem() }
    }

    pub fn get_mask_value(&self, elt: u32) -> i32 {
        unsafe { LLVMGetMaskValue(self.0.as_ptr(), elt) }
    }

    pub fn is_atomic_single_thread(&self) -> bool {
        unsafe { LLVMIsAtomicSingleThread(self.0.as_ptr()) != 0 }
    }

    pub fn set_atomic_single_thread(&self, single_thread: bool) {
        unsafe { LLVMSetAtomicSingleThread(self.0.as_ptr(), single_thread as i32); }
    }

    pub fn get_cmp_xchg_success_ordering(&self) -> AtomicOrdering {
        unsafe { transmute(LLVMGetCmpXchgSuccessOrdering(self.0.as_ptr())) }
    }

    pub fn set_cmp_xchg_success_ordering(&self, ordering: AtomicOrdering) {
        unsafe { LLVMSetCmpXchgSuccessOrdering(self.0.as_ptr(), transmute(ordering)) }
    }

    pub fn get_cmp_xchg_failure_ordering(&self) -> AtomicOrdering {
        unsafe { transmute(LLVMGetCmpXchgFailureOrdering(self.0.as_ptr())) }
    }

    pub fn set_cmp_xchg_failure_ordering(&self, ordering: AtomicOrdering) {
        unsafe { LLVMSetCmpXchgFailureOrdering(self.0.as_ptr(), transmute(ordering)) }
    }

    pub fn get_element_as_constant(&self, index: u32) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetElementAsConstant(self.0.as_ptr(), index)
            )
        })
    }

    pub fn get_const_opcode(&self) -> Opcode {
        unsafe { transmute(LLVMGetConstOpcode(self.0.as_ptr())) }
    }

    // TODO: pub fn ConstVector(values: Vec<Value>) -> Value;
    // TODO: pub fn ConstNeg(&self) -> Value;
    // TODO: pub fn ConstNSWNeg(&self) -> Value;
    // TODO: pub fn ConstNUWNeg(&self) -> Value;
    // TODO: pub fn ConstFNeg(&self) -> Value;
    // TODO: pub fn ConstNot(&self) -> Value;
    // TODO: pub fn ConstAdd(&self) -> Value;
    // TODO: pub fn ConstNSWAdd(&self) -> Value;
    // TODO: pub fn ConstNUWAdd(&self) -> Value;
    // TODO: pub fn ConstFAdd(&self) -> Value;
    // TODO: pub fn ConstSub(&self) -> Value;
    // TODO: pub fn ConstNSWSub(&self) -> Value;
    // TODO: pub fn ConstNUWSub(&self) -> Value;
    // TODO: pub fn ConstFSub(&self) -> Value;
    // TODO: pub fn ConstMul(&self) -> Value;
    // TODO: pub fn ConstNSWMul(&self) -> Value;
    // TODO: pub fn ConstNUWMul(&self) -> Value;
    // TODO: pub fn ConstFMul(&self) -> Value;
    // TODO: pub fn ConstUDiv(&self) -> Value;
    // TODO: pub fn ConstExactUDiv(&self) -> Value;
    // TODO: pub fn ConstSDiv(&self) -> Value;
    // TODO: pub fn ConstExactSDiv(&self) -> Value;
    // TODO: pub fn ConstFDiv(&self) -> Value;
    // TODO: pub fn ConstURem(&self) -> Value;
    // TODO: pub fn ConstSRem(&self) -> Value;
    // TODO: pub fn ConstFRem(&self) -> Value;
    // TODO: pub fn ConstAnd(&self) -> Value;
    // TODO: pub fn ConstOr(&self) -> Value;
    // TODO: pub fn ConstXor(&self) -> Value;
    // TODO: pub fn ConstICmp(predicate: IntPredicate, lhs: Value, rhs: Value) -> Value;
    // TODO: pub fn ConstFCmp(predicate: FloatPredicate, lhs: Value, rhs: Value) -> Value;
    // TODO: pub fn ConstShl(&self) -> Value;
    // TODO: pub fn ConstLShr(&self) -> Value;
    // TODO: pub fn ConstAShr(&self) -> Value;
    // TODO: pub fn ConstGEP2(ty: Type, value: Value, indices: Vec<Value>) -> Value;
    // TODO: pub fn ConstInBoundsGEP2(ty: Type, value: Value, indices: Vec<Value>) -> Value;
    // TODO: pub fn ConstTrunc(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstSExt(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstZExt(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstFPTrunc(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstFPExt(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstUIToFP(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstSIToFP(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstFPToUI(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstFPToSI(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstPtrToInt(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstIntToPtr(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstBitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstAddrSpaceCast(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstZExtOrBitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstSExtOrBitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstTruncOrBitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstPointerCast(&self, to_type: Type) -> Value;

    // TODO: pub fn ConstIntCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef, isSigned: LLVMBool) -> LLVMValueRef;
    // TODO: pub fn ConstFPCast(&self, to_type: Type) -> Value;
    // TODO: pub fn ConstSelect(ConstantCondition: Value, ConstantIfTrue: Value, ConstantIfFalse: Value,) -> LLVMValueRef;
    // TODO: pub fn ConstExtractElement(&self, IndexConstant: Value, ) -> LLVMValueRef;
    // TODO: pub fn ConstInsertElement(&self, ElementValueConstant: Value, IndexConstant: LLVMValueRef, ) -> LLVMValueRef;
    // TODO: pub fn ConstShuffleVector(&self, VectorBConstant: Value, MaskConstant: LLVMValueRef, ) -> LLVMValueRef;
    // TODO: pub fn ConstExtractValue(AggConstant: Value, IdxList: *mut u32, NumIdx: u32, ) -> LLVMValueRef;
    // TODO: pub fn ConstInsertValue(AggConstant: Value, ElementValueConstant: LLVMValueRef, IdxList: *mut u32, NumIdx: u32, ) -> LLVMValueRef;

    pub fn block_address(&self, basic_block: BasicBlock) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMBlockAddress(self.0.as_ptr(), basic_block.as_raw().as_ptr())
            )
        })
    }

    // Core->Values->Constants->Global Values
    pub fn get_global_parent<'a>(&'a self, context: &'a Context) -> Module {
        Module::from_raw(context, unsafe {
            NonNull::new_unchecked(
                LLVMGetGlobalParent(self.0.as_ptr())
            )
        })
    }

    pub fn is_declaration(&self) -> bool {
        unsafe { LLVMIsDeclaration(self.0.as_ptr()) != 0 }
    }

    pub fn get_linkage(&self) -> Linkage {
        unsafe { transmute(LLVMGetLinkage(self.0.as_ptr())) }
    }

    pub fn set_linkage(&self, linkage: Linkage) {
        unsafe { LLVMSetLinkage(self.0.as_ptr(), transmute(linkage)); }
    }

    pub fn get_section(&self) -> &str {
        unsafe { from_c_string(LLVMGetSection(self.0.as_ptr())) }
    }

    pub fn set_section(&self, section: &str) {
        unsafe { LLVMSetSection(self.0.as_ptr(), to_c_string(Some(section)).as_ptr()); }
    }

    pub fn get_visibility(&self) -> Visibility {
        unsafe { transmute(LLVMGetVisibility(self.0.as_ptr())) }
    }

    pub fn set_visibility(&self, visibility: Visibility) {
        unsafe { LLVMSetVisibility(self.0.as_ptr(), transmute(visibility)) }
    }

    pub fn get_dll_storage_class(&self) -> DLLStorageClass {
        unsafe { transmute(LLVMGetDLLStorageClass(self.0.as_ptr())) }
    }

    pub fn set_dll_storage_class(&self, dll_storage_class: DLLStorageClass) {
        unsafe { LLVMSetDLLStorageClass(self.0.as_ptr(), transmute(dll_storage_class)) }
    }

    pub fn get_unnamed_address(&self) -> UnnamedAddr {
        unsafe { transmute(LLVMGetUnnamedAddress(self.0.as_ptr())) }
    }

    pub fn set_unnamed_address(&self, unnamed_addr: UnnamedAddr) {
        unsafe { LLVMSetUnnamedAddress(self.0.as_ptr(), transmute(unnamed_addr)); }
    }

    pub fn global_get_value_type(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGlobalGetValueType(self.0.as_ptr())
            )
        })
    }

    pub fn get_alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.0.as_ptr()) }
    }

    pub fn set_alignment(&self, bytes: u32) {
        unsafe { LLVMSetAlignment(self.0.as_ptr(), bytes) }
    }

    pub fn global_set_metadata(&self, kind: u32, metadata: Metadata) {
        unsafe { LLVMGlobalSetMetadata(self.0.as_ptr(), kind, metadata.as_raw().as_ptr()); }
    }

    pub fn global_erase_metadata(&self, kind: u32) {
        unsafe { LLVMGlobalEraseMetadata(self.0.as_ptr(), kind); }
    }

    pub fn global_clear_metadata(&self) {
        unsafe { LLVMGlobalClearMetadata(self.0.as_ptr()); }
    }

    pub fn get_next_global(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetNextGlobal(self.0.as_ptr())
            )
        })
    }

    pub fn get_previous_global(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetNextGlobal(self.0.as_ptr())
            )
        })
    }

    pub fn delete_global(&self) {
        unsafe { LLVMDeleteGlobal(self.0.as_ptr()) }
    }

    pub fn get_initializer(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetInitializer(self.0.as_ptr())
            )
        })
    }

    pub fn set_initializer(&self, value: Value) {
        unsafe { LLVMSetInitializer(self.0.as_ptr(), value.0.as_ptr()) }
    }

    pub fn is_thread_local(&self) -> bool {
        unsafe { LLVMIsThreadLocal(self.0.as_ptr()) != 0 }
    }

    pub fn set_thread_local(&self, is_thread_local: bool) {
        unsafe { LLVMSetThreadLocal(self.0.as_ptr(), is_thread_local as i32); }
    }

    pub fn is_global_constant(&self) -> bool {
        unsafe { LLVMIsGlobalConstant(self.0.as_ptr()) != 0 }
    }

    pub fn set_global_constant(&self, is_constant: bool) {
        unsafe { LLVMSetGlobalConstant(self.0.as_ptr(), is_constant as i32); }
    }

    pub fn get_thread_local_mode(&self) -> ThreadLocalMode {
        unsafe { transmute(LLVMGetThreadLocalMode(self.0.as_ptr())) }
    }

    pub fn set_thread_local_mode(&self, thread_local_mode: ThreadLocalMode) {
        unsafe { LLVMSetThreadLocalMode(self.0.as_ptr(), transmute(thread_local_mode)) }
    }

    pub fn is_externally_initialized(&self) -> bool {
        unsafe { LLVMIsExternallyInitialized(self.0.as_ptr()) != 0 }
    }

    pub fn set_externally_initialized(&self, is_externally_initialized: bool) {
        unsafe { LLVMSetExternallyInitialized(self.0.as_ptr(), is_externally_initialized as i32) }
    }

    pub fn get_value_kind(&self) -> ValueKind {
        unsafe { transmute(LLVMGetValueKind(self.0.as_ptr())) }
    }

    pub fn type_of(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMTypeOf(self.0.as_ptr())
            )
        })
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let chars = LLVMGetValueName2(self.0.as_ptr(), len.as_mut_ptr());

            String::from_utf8(
                transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init())).to_vec()
            ).unwrap()
        }
    }

    pub fn set_name(&self, name: &str) {
        unsafe { LLVMSetValueName2(self.0.as_ptr(), to_c_string(Some(name)).as_ptr(), name.len()); }
    }

    pub fn get_next_global_alias(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetNextGlobalAlias(self.0.as_ptr())
            )
        })
    }

    pub fn get_previous_global_alias(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetPreviousGlobalAlias(self.0.as_ptr())
            )
        })
    }

    pub fn alias_get_aliasee(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMAliasGetAliasee(self.0.as_ptr())
            )
        })
    }

    pub fn alias_set_aliasee(&self, aliasee: Value) {
        unsafe {
            LLVMAliasSetAliasee(self.0.as_ptr(), aliasee.0.as_ptr());
        }
    }

    pub fn dump_value(&self) {
        unsafe { LLVMDumpValue(self.0.as_ptr()); }
    }

    pub fn print_value_to_string(&self) -> &str {
        unsafe { from_c_string(LLVMPrintValueToString(self.0.as_ptr())) }
    }

    pub fn replace_all_uses_with(&self, new: Value) {
        unsafe { LLVMReplaceAllUsesWith(self.0.as_ptr(), new.0.as_ptr()) }
    }

    pub fn is_constant(&self) -> bool {
        unsafe { LLVMIsConstant(self.0.as_ptr()) != 0 }
    }

    pub fn is_undef(&self) -> bool {
        unsafe { LLVMIsUndef(self.0.as_ptr()) != 0 }
    }

    pub fn is_poison(&self) -> bool {
        unsafe { LLVMIsPoison(self.0.as_ptr()) != 0 }
    }

    pub fn is_amd_node(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMIsAMDNode(self.0.as_ptr())
            )
        })
    }

    pub fn is_amd_string(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMIsAMDString(self.0.as_ptr())
            )
        })
    }

    pub fn get_operand(&self, index: u32) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetOperand(self.0.as_ptr(), index)
            )
        })
    }

    pub fn get_operand_use(&self, index: u32) -> Use {
        Use::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetOperandUse(self.0.as_ptr(), index)
            )
        })
    }

    pub fn set_operand(&self, index: u32, operand: Value) {
        unsafe { LLVMSetOperand(self.0.as_ptr(), index, operand.0.as_ptr()); }
    }

    pub fn get_num_operands(&self) -> i32 {
        unsafe { LLVMGetNumOperands(self.0.as_ptr()) }
    }

    // TODO: pub fn LLVMLookupIntrinsicID(Name: *const ::libc::c_char, NameLen: ::libc::size_t, ) -> u32;

    // TODO: pub fn LLVMGetIntrinsicDeclaration(Mod: LLVMModuleRef, ID: u32, ParamTypes: *mut LLVMTypeRef, ParamCount: ::libc::size_t, ) -> LLVMValueRef;
    // TODO: pub fn LLVMIntrinsicGetType(Ctx: LLVMContextRef, ParamTypes: *mut LLVMTypeRef, ParamCount: ::libc::size_t, ) -> LLVMTypeRef;
    // TODO: pub fn LLVMIntrinsicGetName(ID: u32, NameLength: *mut ::libc::size_t, ) -> *const ::libc::c_char;
    // TODO: pub fn LLVMIntrinsicCopyOverloadedName2(Mod: LLVMModuleRef, ID: u32, ParamTypes: *mut LLVMTypeRef, ParamCount: ::libc::size_t, NameLength: *mut ::libc::size_t, ) -> *const ::libc::c_char;
    // TODO: pub fn LLVMIntrinsicIsOverloaded(ID: u32) -> LLVMBool;

    // TODO: pub fn AddAttributeAtIndex(&self, Idx: LLVMAttributeIndex, A: LLVMAttributeRef);
    // TODO: pub fn GetAttributeCountAtIndex(&self, Idx: LLVMAttributeIndex) -> u32;

    // TODO: pub fn LLVMGetAttributesAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex, Attrs: *mut LLVMAttributeRef, );
    // TODO: pub fn LLVMGetEnumAttributeAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex, KindID: u32, ) -> LLVMAttributeRef;
    // TODO: pub fn LLVMGetStringAttributeAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex, K: *const ::libc::c_char, KLen: u32, ) -> LLVMAttributeRef;
    // TODO: pub fn LLVMRemoveEnumAttributeAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex, KindID: u32, );
    // TODO: pub fn LLVMRemoveStringAttributeAtIndex(F: LLVMValueRef, Idx: LLVMAttributeIndex, K: *const ::libc::c_char, KLen: u32, );
    // TODO: pub fn LLVMAddTargetDependentFunctionAttr(Fn: LLVMValueRef, A: *const ::libc::c_char, V: *const ::libc::c_char, );

    pub fn as_basic_block(&self) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(LLVMValueAsBasicBlock(self.0.as_ptr()))
        })
    }

    pub fn append_existing_basic_block(&self, basic_block: BasicBlock) {
        unsafe { LLVMAppendExistingBasicBlock(self.0.as_ptr(), basic_block.as_raw().as_ptr()); }
    }

    pub fn append_basic_block(&self, name: &str) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMAppendBasicBlock(self.0.as_ptr(), to_c_string(Some(name)).as_ptr())
            )
        })
    }

    pub fn is_constant_string(&self) -> bool {
        unsafe { LLVMIsConstantString(self.0.as_ptr()) != 0 }
    }

    pub fn as_string(&self) -> String {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let chars = LLVMGetAsString(self.0.as_ptr(), len.as_mut_ptr());

            String::from_utf8(
                transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init())).to_vec()
            ).unwrap()
        }
    }

    pub fn as_const_int_zext(&self) -> u64 {
        unsafe { LLVMConstIntGetZExtValue(self.0.as_ptr()) }
    }

    pub fn as_const_int_sext(&self) -> i64 {
        unsafe { LLVMConstIntGetSExtValue(self.0.as_ptr()) }
    }

    pub fn as_double(&self) -> (f64, bool) {
        unsafe {
            let mut loses_info = MaybeUninit::uninit();
            let double = LLVMConstRealGetDouble(self.0.as_ptr(), loses_info.as_mut_ptr());

            (double, loses_info.assume_init() != 0)
        }
    }

    // pub fn LLVMGlobalCopyAllMetadata(&self) -> *mut LLVMValueMetadataEntry;

    pub fn get_first_use(&self) -> Use {
        Use::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetFirstUse(self.0.as_ptr()))
        })
    }
}

pub struct Use(NonNull<LLVMUse>);

impl Use {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMUse>) -> Use {
        Use(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMUse> {
        self.0
    }
    
    pub fn get_next_use(&self) -> Use {
        Use::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetNextUse(self.0.as_ptr()))
        })
    }
    
    pub fn get_user(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetUser(self.0.as_ptr()))
        })
    }
    
    pub fn get_used_value(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetUsedValue(self.0.as_ptr()))
        })
    }
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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnnamedAddr {
    None,
    Local,
    GlobalUnnamedAddr,
}


#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueKind {
    Argument,
    BasicBlock,
    MemoryUse,
    MemoryDef,
    MemoryPhi,
    Function,
    GlobalAlias,
    GlobalIFunc,
    GlobalVariable,
    BlockAddress,
    ConstantExpr,
    ConstantArray,
    ConstantStruct,
    ConstantVector,
    UndefValue,
    ConstantAggregateZero,
    ConstantDataArray,
    ConstantDataVector,
    ConstantInt,
    ConstantFP,
    ConstantPointerNull,
    ConstantTokenNone,
    MetadataAsValue,
    InlineAsm,
    Instruction,
    Poison,
}

// pub fn LLVMDisposeValueMetadataEntries(Entries: *mut LLVMValueMetadataEntry);
// pub fn LLVMValueMetadataEntriesGetKind(Entries: *mut LLVMValueMetadataEntry, Index: u32) -> u32;
// pub fn LLVMValueMetadataEntriesGetMetadata(Entries: *mut LLVMValueMetadataEntry, Index: u32) -> LLVMMetadataRef;

// pub fn LLVMDisposeModuleProvider(M: LLVMModuleProviderRef);
// pub fn LLVMGetGlobalPassRegistry() -> LLVMPassRegistryRef;