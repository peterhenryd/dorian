use std::ptr::NonNull;

use crate::llvm::basic_block::BasicBlock;
use crate::llvm::opcode::Opcode;
use crate::llvm::types::Type;
use crate::llvm::{to_c_string, AtomicOrdering, AtomicRMWBinOp, FloatPredicate, IntPredicate};
use crate::llvm::debug::Metadata;
use crate::llvm::fun::Fun;

use crate::llvm::sys::core::*;
use crate::llvm::sys::prelude::LLVMBool;
use crate::llvm::sys::*;
use crate::llvm::value::Value;

/// Encapsulates the [LLVMBuilder] and its functions.
pub struct Builder(NonNull<LLVMBuilder>);

impl Builder {
    /// Creates a [Builder] from an [LLVMBuilder].
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMBuilder>) -> Builder {
        Builder(raw)
    }

    /// Borrow the internal [LLVMBuilder].
    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMBuilder> {
        self.0
    }

    // TODO: explain what this does
    pub fn position_at_end(&self, block: &BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.0.as_ptr(), block.as_raw().as_ptr());
        }
    }

    pub fn build_add(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_add(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNSWAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_add(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNUWAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_add(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFAdd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_sub(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_sub(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNSWSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_sub(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNUWSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_sub(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFSub(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_mul(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_mul(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNSWMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_mul(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNUWMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_mul(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFMul(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_u_div(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildUDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_exact_u_div(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildExactUDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_div(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildSDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_exact_s_div(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildExactSDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_div(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFDiv(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_u_rem(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildURem(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_rem(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildSRem(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_rem(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFRem(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_shl(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildShl(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_l_shr(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildLShr(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_a_shr(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildAShr(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_and(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildAnd(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_or(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildOr(
                self.0.as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_xor(&self, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
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
        &self,
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

    pub fn build_neg(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nsw_neg(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_nuw_neg(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_f_neg(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_not(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildNeg(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_malloc(&self, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildMalloc(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_array_malloc(
        &self,
        ty: Type,
        value: Value,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildArrayMalloc(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_mem_set(&self, ptr: Value, value: Value, len: Value, align: u32) -> Value {
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
        &self,
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
        &self,
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

    pub fn build_alloca(&self, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildAlloca(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_array_alloca(&self, ty: Type, len: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildArrayAlloca(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                len.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_free(&self, ptr: Value) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFree(
                self.0.as_ptr(),
                ptr.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_load(&self, ty: Type, ptr: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildLoad2(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                ptr.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_store(&self, value: Value, ptr: Value) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildStore(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ptr.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_gep(&self, ty: Type, ptr: Value, indices: Vec<Value>, name: Option<&str>) -> Value {
        let len = indices.len();

        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildGEP2(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
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
        &self,
        ty: Type,
        ptr: Value,
        indices: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = indices.len();

        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildInBoundsGEP2(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
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

    pub fn build_global_string(&self, string: &str, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildGlobalString(
                self.0.as_ptr(),
                to_c_string(Some(string)).as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_global_string_ptr(&self, string: &str, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildGlobalStringPtr(
                self.0.as_ptr(),
                to_c_string(Some(string)).as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_trunc(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildTrunc(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_z_ext(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildZExt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_ext(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildSExt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_to_ui(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFPToUI(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_to_si(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFPToSI(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ui_to_fp(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildUIToFP(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_si_to_fp(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildSIToFP(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_trunc(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFPTrunc(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_ext(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFPExt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ptr_to_int(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildPtrToInt(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_int_to_ptr(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildIntToPtr(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_bit_cast(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_addr_space_cast(
        &self,
        value: Value,
        ty: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildAddrSpaceCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_z_ext_or_bit_cast(
        &self,
        value: Value,
        ty: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildZExtOrBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_s_ext_or_bit_cast(
        &self,
        value: Value,
        ty: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildSExtOrBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_trunc_or_bit_cast(
        &self,
        value: Value,
        ty: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildTruncOrBitCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_cast(
        &self,
        op: Opcode,
        value: Value,
        ty: Type,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildCast(
                self.0.as_ptr(),
                std::mem::transmute(op),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ptr_cast(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildPointerCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_int_cast(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildIntCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_int_cast_2(
        &self,
        value: Value,
        ty: Type,
        is_signed: bool,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildIntCast2(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                is_signed as LLVMBool,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fp_cast(&self, value: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFPCast(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_i_cmp(
        &self,
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
        &self,
        op: FloatPredicate,
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

    pub fn build_phi(&self, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildPhi(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_call(
        &self,
        ty: Type,
        fun: Fun,
        args: Vec<Value>,
        name: Option<&str>,
    ) -> Value {
        let len = args.len();

        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildCall2(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                fun.as_raw().as_ptr(),
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
        &self,
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

    pub fn build_va_arg(&self, vector: Value, ty: Type, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildVAArg(
                self.0.as_ptr(),
                vector.as_raw().as_ptr(),
                ty.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_extract_element(
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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

    pub fn build_freeze(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildFreeze(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_is_null(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildIsNull(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_is_not_null(&self, value: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildIsNotNull(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_ptr_diff(&self, ty: Type, lhs: Value, rhs: Value, name: Option<&str>) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildPtrDiff2(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                lhs.as_raw().as_ptr(),
                rhs.as_raw().as_ptr(),
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_fence(
        &self,
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
        &self,
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
        &self,
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

    pub fn build_ret_void(&self) -> Value {
        unsafe { Value::from_raw(NonNull::new_unchecked(LLVMBuildRetVoid(self.0.as_ptr()))) }
    }

    pub fn build_ret(&self, value: Value) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildRet(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_aggregate_ret(&self, values: Vec<Value>) -> Value {
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

    pub fn build_br(&self, dest: BasicBlock) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildBr(
                self.0.as_ptr(),
                dest.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_cond_br(&self, r#if: Value, then: BasicBlock, r#else: BasicBlock) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildCondBr(
                self.0.as_ptr(),
                r#if.as_raw().as_ptr(),
                then.as_raw().as_ptr(),
                r#else.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_switch(&self, value: Value, r#else: BasicBlock, cases: u32) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildSwitch(
                self.0.as_ptr(),
                value.as_raw().as_ptr(),
                r#else.as_raw().as_ptr(),
                cases,
            )))
        }
    }

    pub fn build_indirect_br(&self, addr: Value, dests: u32) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildIndirectBr(
                self.0.as_ptr(),
                addr.as_raw().as_ptr(),
                dests,
            )))
        }
    }

    pub fn build_invoke(
        &self,
        ty: Type,
        fun: Value,
        args: Vec<Value>,
        then: BasicBlock,
        catch: BasicBlock,
        name: Option<&str>,
    ) -> Value {
        let len = args.len();

        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildInvoke2(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                fun.as_raw().as_ptr(),
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

    pub fn build_unreachable(&self) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildUnreachable(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn build_resume(&self, exn: Value) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildResume(
                self.0.as_ptr(),
                exn.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_landing_pad(
        &self,
        ty: Type,
        pers_fn: Value,
        clauses: u32,
        name: Option<&str>,
    ) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildLandingPad(
                self.0.as_ptr(),
                ty.as_raw().as_ptr(),
                pers_fn.as_raw().as_ptr(),
                clauses,
                to_c_string(name).as_ptr(),
            )))
        }
    }

    pub fn build_cleanup_ret(&self, catch_pad: Value, block: BasicBlock) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildCleanupRet(
                self.0.as_ptr(),
                catch_pad.as_raw().as_ptr(),
                block.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_catch_ret(&self, catch_pad: Value, block: BasicBlock) -> Value {
        unsafe {
            Value::from_raw(NonNull::new_unchecked(LLVMBuildCatchRet(
                self.0.as_ptr(),
                catch_pad.as_raw().as_ptr(),
                block.as_raw().as_ptr(),
            )))
        }
    }

    pub fn build_catch_pad(
        &self,
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
        &self,
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
        &self,
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

    pub fn insert_existing_basic_block_after_insert_block(&self, basic_block: BasicBlock) {
        unsafe {
            LLVMInsertExistingBasicBlockAfterInsertBlock(self.0.as_ptr(), basic_block.as_raw().as_ptr())
        }
    }

    pub fn get_current_debug_location(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetCurrentDebugLocation2(self.0.as_ptr())
            )
        })
    }

    pub fn get_current_debug_location_value(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetCurrentDebugLocation(self.0.as_ptr())
            )
        })
    }

    pub fn set_current_debug_location(&self, loc: Metadata) {
        unsafe {
            LLVMSetCurrentDebugLocation2(self.0.as_ptr(), loc.as_raw().as_ptr())
        }
    }

    pub fn add_metadata_to_inst(&self, inst: Value) {
        unsafe { LLVMAddMetadataToInst(self.0.as_ptr(), inst.as_raw().as_ptr()); }
    }

    pub fn get_default_fp_math_tag(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMBuilderGetDefaultFPMathTag(self.0.as_ptr())
            )
        })
    }

    pub fn set_default_fp_math_tag(&self, metadata: Metadata) {
        unsafe {
            LLVMBuilderSetDefaultFPMathTag(self.0.as_ptr(), metadata.as_raw().as_ptr());
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
