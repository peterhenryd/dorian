use std::borrow::Cow;
use crate::block::Block;
use crate::val::{Value, Var};

/*
[x] build_return
NP  build_aggregate_return
[x] build_call
NP  build_direct_call
[ ] build_direct_call_with_operand_bundles
[ ] build_indirect_call
[ ] build_indirect_call_with_operand_bundles
[ ] build_call_help
[ ] build_call_with_operand_bundles_help
[ ] build_invoke
[ ] build_direct_invoke
[ ] build_indirect_invoke
[ ] build_invoke_help
[ ] build_landing_pad
[ ] build_resume
[ ] build_gep
[ ] build_in_bounds_gep
[ ] build_struct_gep
[ ] build_ptr_diff
[ ] build_phi
[ ] build_store
[ ] build_load
[ ] build_alloca
[ ] build_array_alloca
[ ] build_memcpy
[ ] build_memmove
[ ] build_memset
[ ] build_malloc
[ ] build_array_malloc
[ ] build_free
[ ] built_int_unsigned_div
[ ] build_int_signed_div
[ ] built_int_exact_signed_div
[ ] build_int_unsigned_rem
[ ] build_int_signed_rem
[ ] build_int_s_extend
[ ] build_address_space_cast
[ ] build_bit_cast
[ ] built_int_s_extend_or_bit_cast
[ ] build_int_z_extend
[ ] build_int_z_extend_or_bit_cast
[ ] build_int_truncate
[ ] build_int_truncate_or_bit_cast
[ ] build_float_rem
[ ] build_float_to_unsigned_int
[ ] build_float_to_signed_int
[ ] build_unsigned_int_to_float
[ ] build_signed_int_to_float
[ ] build_float_trunc
[ ] built_float_ext
[ ] build_float_cast
[ ] build_int_cast
[ ] built_int_cast_sign_flag
[ ] build_float_div
[ ] build_int_nsw_add
[ ] build_int_nuw_add
[ ] built_float_add
[ ] build_xor
[ ] build_add
[ ] build_or
[ ] build_left_shift
[ ] build_right_shift
[ ] build_int_sub
[ ] build_int_nsw_sub
[ ] build_int_nuw_sub
[ ] build_float_sub
[ ] build_int_mul
[ ] build_int_nsw_mul
[ ] build_int_nuw_mul
[ ] build_float_mul
[ ] build_binop
[ ] build_cast
[ ] build_pointer_cast
[ ] build_int_compare
[ ] build_float_compare
[ ] build_unconditional_branch
[ ] build_conditional_branch
[ ] build_indirect_branch
[ ] build_int_neg
[ ] build_int_nsw_neg
[ ] build_int_nuw_neg
[ ] build_float_neg
[ ] build_not
[ ] build_extract_value
[ ] build_insert_value
[ ] build_extract_element
[ ] build_insert_element
[ ] build_unreachable
[ ] build_fence
[ ] build_is_null
[ ] build_is_not_null
[ ] build_int_to_ptr
[ ] build_ptr_to_int
[ ] build_switch
[ ] build_select
[ ] build_global_string
[ ] build_global_string_ptr
[ ] build_shuffle_vector
[ ] build_va_arg
[ ] build_atomicrmw
[ ] build_cmpxchg
 */

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'s> {
    If(IfStmt<'s>),
    While(WhileStmt<'s>),
    Return(ReturnStmt<'s>),
    Bind(BindStmt<'s>),
    Assign(AssignStmt<'s>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStmt<'s> {
    pub condition: Value<'s>,
    pub then_block: Block<'s>,
    pub if_else: Option<IfElse<'s>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfElse<'s> {
    If(Box<IfStmt<'s>>),
    Else(Block<'s>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt<'s> {
    pub condition: Value<'s>,
    pub loop_block: Block<'s>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt<'s> {
    pub value: Option<Value<'s>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindStmt<'s> {
    pub name: Cow<'s, str>,
    pub value: Value<'s>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignStmt<'s> {
    pub var: Var<'s>,
    pub value: Value<'s>,
}