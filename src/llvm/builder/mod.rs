use std::ptr::NonNull;

use crate::llvm::basic_block::BasicBlock;
use crate::llvm::opcode::Opcode;
use crate::llvm::types::Type;
use crate::llvm::{to_c_string, AtomicOrdering, AtomicRMWBinOp, IntPredicate, RealPredicate};

use crate::llvm::sys::core::*;
use crate::llvm::sys::prelude::LLVMBool;
use crate::llvm::sys::*;
use crate::llvm::value::Value;
use crate::function::block::Block;

pub struct Builder(NonNull<LLVMBuilder>);

impl Builder {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMBuilder>) -> Builder {
        Builder(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMBuilder> {
        self.0
    }

    pub fn position_at_end(&mut self, block: Block) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.0.as_ptr(), block.get_llvm().as_raw().as_ptr());
        }
    }

    pub fn build_add(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_add(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNSWAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_add(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw( NonNull::new_unchecked(LLVMBuildNUWAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_add(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_sub(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_sub(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNSWSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_sub(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNUWSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_sub(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_mul(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_mul(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNSWMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_mul(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNUWMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_mul(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_u_div(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildUDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_exact_u_div(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildExactUDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_div(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_exact_s_div(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildExactSDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_div(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_u_rem(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildURem(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_rem(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSRem(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_rem(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFRem(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_shl(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildShl(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_l_shr(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildLShr(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_a_shr(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAShr(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_and(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAnd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_or(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildOr(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_xor(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildXor(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_bin_op(
        &mut self,
        op: LLVMOpcode,
        lhs: Value,
        rhs: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildBinOp(
                self.0.as_ptr(),
                op as LLVMOpcode,
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_neg(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_neg(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_neg(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_neg(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_not(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_malloc(&mut self, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildMalloc(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_array_malloc(&mut self, r#type: Type, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildArrayMalloc(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_mem_set(&mut self, ptr: Value, value: Value, len: Value, align: u32) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildMemSet(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
                value.as_raw().as_ptr(),
                len.as_raw().as_ptr(),
                align,
            )))
        }
    }

    pub fn build_mem_cpy(
        &mut self,
        dst: Value,
        dst_align: u32,
        src: Value,
        src_align: u32,
        size: Value,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildMemCpy(
                self.0.as_ptr(),
                dst.as_raw().as_ptr(),
                dst_align,
                src.as_raw().as_ptr(),
                src_align,
                size.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_mem_move(
        &mut self,
        dst: Value,
        dst_align: u32,
        src: Value,
        src_align: u32,
        size: Value,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildMemMove(
                self.0.as_ptr(),
                dst.as_raw().as_ptr(),
                dst_align,
                src.as_raw().as_ptr(),
                src_align,
                size.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_alloca(&mut self, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAlloca(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_array_alloca(&mut self, r#type: Type, len: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildArrayAlloca(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                len.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_free(&mut self, ptr: Value) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFree(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_load(&mut self, ptr: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildLoad(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_load_2(&mut self, r#type: Type, ptr: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildLoad2(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                ptr.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_store(&mut self, value: Value, ptr: Value) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildStore(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ptr.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_gep(&mut self, ptr: Value, indices: Vec<Value>, name: Option<&str>) -> Value {
        let len = indices.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildGEP(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
                indices
                    .into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_in_bounds_gep(
        &mut self,
        ptr: Value,
        indices: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = indices.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildInBoundsGEP(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
                indices
                    .into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    // TODO: soundness?
    pub fn build_struct_gep(&mut self, ptr: Value, index: u32, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildStructGEP(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
                index,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_gep_2(
        &mut self,
        r#type: Type,
        ptr: Value,
        indices: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = indices.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildGEP2(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                ptr.as_raw().as_ptr(),
                indices
                    .into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_in_bounds_gep_2(
        &mut self,
        r#type: Type,
        ptr: Value,
        indices: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = indices.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildInBoundsGEP2(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                ptr.as_raw().as_ptr(),
                indices
                    .into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_struct_gep_2(
        &mut self,
        r#type: Type,
        ptr: Value,
        index: u32,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildStructGEP2(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                ptr.as_raw().as_ptr(),
                index,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_global_string(&mut self, string: &str, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildGlobalString(
                self.0.as_ptr(),
                to_c_string(Some(string)).as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_global_string_ptr(&mut self, string: &str, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildGlobalStringPtr(
                self.0.as_ptr(),
                to_c_string(Some(string)).as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_trunc(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildTrunc(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_z_ext(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildZExt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_ext(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSExt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_to_ui(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFPToUI(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_to_si(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFPToSI(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ui_to_fp(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildUIToFP(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_si_to_fp(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSIToFP(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_trunc(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFPTrunc(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_ext(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFPExt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ptr_to_int(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildPtrToInt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn built_int_to_ptr(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildIntToPtr(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_bit_cast(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_addr_space_cast(
        &mut self,
        value: Value,
        r#type: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAddrSpaceCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_z_ext_or_bit_cast(
        &mut self,
        value: Value,
        r#type: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildZExtOrBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_ext_or_bit_cast(
        &mut self,
        value: Value,
        r#type: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSExtOrBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_trunc_or_bit_cast(
        &mut self,
        value: Value,
        r#type: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildTruncOrBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_cast(
        &mut self,
        op: Opcode,
        value: Value,
        r#type: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCast(
                self.0.as_ptr(),
                std::mem::transmute(op),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ptr_cast(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildPointerCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_int_cast(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildIntCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_int_cast_2(
        &mut self,
        value: Value,
        r#type: Type,
        is_signed: bool,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildIntCast2(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                is_signed as LLVMBool,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_cast(&mut self, value: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFPCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_i_cmp(
        &mut self,
        op: IntPredicate,
        lhs: Value,
        rhs: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildICmp(
                self.0.as_ptr(),
                std::mem::transmute(op),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_cmp(
        &mut self,
        op: RealPredicate,
        lhs: Value,
        rhs: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFCmp(
                self.0.as_ptr(),
                std::mem::transmute(op),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_phi(&mut self, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildPhi(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_call(&mut self, function: Value, args: impl Iterator<Item = Value> + ExactSizeIterator, name: Option<&str>) -> Value {
        let len = args.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCall(
                self.0.as_ptr(),
                function.as_raw().as_ptr(),
                args.into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_call_2(
        &mut self,
        r#type: Type,
        function: Value,
        args: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = args.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCall2(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                function.as_raw().as_ptr(),
                args.into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_select(
        &mut self,
        r#if: Value,
        then: Value,
        r#else: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSelect(
                self.0.as_ptr(),
                r#if.as_raw().as_ptr(),
                then.as_raw().as_ptr(),
                r#else.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_va_arg(&mut self, vector: Value, r#type: Type, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildVAArg(
                self.0.as_ptr(),
                vector.as_raw().as_ptr(),
                r#type.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_extract_element(
        &mut self,
        vector: Value,
        index: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildExtractElement(
                self.0.as_ptr(),
                vector.as_raw().as_ptr(),
                index.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_insert_element(
        &mut self,
        vector: Value,
        element: Value,
        index: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildInsertElement(
                self.0.as_ptr(),
                vector.as_raw().as_ptr(),
                element.as_raw().as_ptr(),
                index.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_shuffle_vector(
        &mut self,
        v1: Value,
        v2: Value,
        mask: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildShuffleVector(
                self.0.as_ptr(),
                v1.as_raw().as_ptr(),
                v2.as_raw().as_ptr(),
                mask.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_extract_value(
        &mut self,
        aggregate: Value,
        index: u32,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildExtractValue(
                self.0.as_ptr(),
                aggregate.as_raw().as_ptr(),
                index,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_insert_value(
        &mut self,
        aggregate: Value,
        element: Value,
        index: u32,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildInsertValue(
                self.0.as_ptr(),
                aggregate.as_raw().as_ptr(),
                element.as_raw().as_ptr(),
                index,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_freeze(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFreeze(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_is_null(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildIsNull(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_is_not_null(&mut self, value: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildIsNotNull(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ptr_diff(&mut self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildPtrDiff(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fence(
        &mut self,
        ordering: AtomicOrdering,
        single_thread: bool,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildFence(
                self.0.as_ptr(),
                std::mem::transmute(ordering),
                single_thread as LLVMBool,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_atomic_rmw(
        &mut self,
        op: AtomicRMWBinOp,
        ptr: Value,
        value: Value,
        ordering: AtomicOrdering,
        single_thread: bool,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAtomicRMW(
                self.0.as_ptr(),
                std::mem::transmute(op),
                ptr.as_raw().as_ptr(),
                value.as_raw().as_ptr(),
                std::mem::transmute(ordering),
                single_thread as LLVMBool,
            )))
        }
    }

    pub fn build_atomic_cmp_xchg(
        &mut self,
        ptr: Value,
        cmp: Value,
        new: Value,
        success_ordering: AtomicOrdering,
        failure_ordering: AtomicOrdering,
        single_thread: bool,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAtomicCmpXchg(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
                cmp.as_raw().as_ptr(),
                new.as_raw().as_ptr(),
                std::mem::transmute(success_ordering),
                std::mem::transmute(failure_ordering),
                single_thread as LLVMBool,
            )))
        }
    }

    pub fn build_ret_void(&mut self) -> Value {
        unsafe {  Value::from_raw(NonNull::new_unchecked(LLVMBuildRetVoid(self.0.as_ptr()))) }
    }

    pub fn build_ret(&mut self, value: Value) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildRet(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_aggregate_ret(&mut self, values: Vec<Value>) -> Value {
        let len = values.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildAggregateRet(
                self.0.as_ptr(),
                values
                    .into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
            )))
        }
    }

    pub fn build_br(&mut self, dest: BasicBlock) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildBr(
                self.0.as_ptr(),
                dest.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_cond_br(&mut self, r#if: Value, then: BasicBlock, r#else: BasicBlock) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCondBr(
                self.0.as_ptr(),
                r#if.as_raw().as_ptr(),
                then.as_raw().as_ptr(),
                r#else.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_switch(&mut self, value: Value, r#else: BasicBlock, cases: u32) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildSwitch(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#else.as_raw().as_ptr(),
                cases,
            )))
        }
    }

    pub fn build_indirect_br(&mut self, addr: Value, dests: u32) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildIndirectBr(
                self.0.as_ptr(),
                addr.as_raw().as_ptr(),
                dests,
            )))
        }
    }

    pub fn build_invoke(
        &mut self,
        r#fn: Value,
        args: Vec<Value>,
        then: BasicBlock,
        catch: BasicBlock,
        name: Option<&str>,
    ) -> Value {
        let len = args.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildInvoke(
                self.0.as_ptr(),
                r#fn.as_raw().as_ptr(),
                args.into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                then.as_raw().as_ptr(),
                catch.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_invoke_2(
        &mut self,
        r#type: Type,
        r#fn: Value,
        args: Vec<Value>,
        then: BasicBlock,
        catch: BasicBlock,
        name: Option<&str>,
    ) -> Value {
        let len = args.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildInvoke2(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                r#fn.as_raw().as_ptr(),
                args.into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                then.as_raw().as_ptr(),
                catch.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_unreachable(&mut self) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildUnreachable(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn build_resume(&mut self, exn: Value) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildResume(
                self.0.as_ptr(),
                exn.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_landing_pad(
        &mut self,
        r#type: Type,
        pers_fn: Value,
        clauses: u32,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildLandingPad(
                self.0.as_ptr(),
                r#type.as_raw().as_ptr(),
                pers_fn.as_raw().as_ptr(),
                clauses,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_cleanup_ret(&mut self, catch_pad: Value, block: BasicBlock) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCleanupRet(
                self.0.as_ptr(),
                catch_pad.as_raw().as_ptr(),
                block.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_catch_ret(&mut self, catch_pad: Value, block: BasicBlock) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCatchRet(
                self.0.as_ptr(),
                catch_pad.as_raw().as_ptr(),
                block.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_catch_pad(
        &mut self,
        parent_pad: Value,
        args: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = args.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCatchPad(
                self.0.as_ptr(),
                parent_pad.as_raw().as_ptr(),
                args.into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_cleanup_pad(
        &mut self,
        parent_pad: Value,
        args: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = args.len();
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCleanupPad(
                self.0.as_ptr(),
                parent_pad.as_raw().as_ptr(),
                args.into_iter()
                    .map(|value| value.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_catch_switch(
        &mut self,
        parent_pad: Value,
        unwind_block: BasicBlock,
        num_handler: u32,
        name: Option<&str>,
    ) -> Value {
        unsafe {
             Value::from_raw(NonNull::new_unchecked(LLVMBuildCatchSwitch(
                self.0.as_ptr(),
                parent_pad.as_raw().as_ptr(),
                unwind_block.as_raw().as_ptr(),
                num_handler,
                to_c_string(name).as_ptr(),
            )))
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.0.as_ptr());
        }
    }
}
