use std::alloc::{alloc, Layout};
use crate::llvm::sys::core::{LLVMConstInt, LLVMConstIntOfStringAndSize, LLVMGetTypeKind, LLVMPointerType, LLVMGetElementType, LLVMArrayType};
use crate::llvm::sys::{LLVMType, LLVMTypeKind};
use crate::llvm::value::Value;
use crate::llvm::{to_c_string, AddressSpace, InlineAsmDialect, from_c_string};
use std::mem::transmute;
use std::ptr::NonNull;
use llvm_sys::core::*;
use crate::llvm::context::Context;
use crate::types::LlvmType;

#[derive(Debug, Copy, Clone)]
pub struct Type(NonNull<LLVMType>);

impl Type {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMType>) -> Type {
        Type(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMType> {
        self.0
    }

    pub fn to_ptr_type(&self, address_space: AddressSpace) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMPointerType(self.0.as_ptr(), transmute(address_space)))
        })
    }

    pub unsafe fn get_pointing_type(&self) -> Type {
        Type::from_raw(
            NonNull::new_unchecked(
                LLVMGetElementType(self.0.as_ptr())
            )
        )
    }

    pub fn get_array_type(&self, size: u32) -> Type {
        Type::from_raw(
            unsafe {
                NonNull::new_unchecked(
                    LLVMArrayType(self.0.as_ptr(), size)
                )
            }
        )
    }

    pub fn get_inline_asm(&self, asm: &str,
                          constraints: &str,
                          has_side_effects: bool,
                          is_align_stack: bool,
                          dialect: InlineAsmDialect,
                          can_throw: bool,
    ) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetInlineAsm(
                    self.0.as_ptr(),
                    to_c_string(Some(asm)).into_raw(),
                    asm.len(),
                    to_c_string(Some(constraints)).into_raw(),
                    constraints.len(),
                    has_side_effects as i32,
                    is_align_stack as i32,
                    transmute(dialect),
                    can_throw as i32
                )
            )
        })
    }

    pub fn get_type_kind(&self) -> TypeKind {
        unsafe { transmute(LLVMGetTypeKind(self.0.as_ptr())) }
    }
    
    pub fn type_is_sized(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.0.as_ptr()) != 0 }
    }
    
    pub fn get_type_context(&self) -> Context {
        Context::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetTypeContext(self.0.as_ptr())
            )
        })
    }
    
    pub fn dump_type(&self) {
        unsafe { LLVMDumpType(self.0.as_ptr()) }
    }
    
    pub fn print_type_to_string(&self) -> &str {
        unsafe { from_c_string(LLVMPrintTypeToString(self.0.as_ptr())) }
    }
    
    pub fn get_int_type_width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.0.as_ptr()) }
    }

    pub fn get_struct_name(&self) -> &str {
        unsafe { from_c_string(LLVMGetStructName(self.0.as_ptr())) }
    }
    
    pub fn struct_set_body(&self, elements: Vec<Type>, packed: bool) {
        unsafe {
            LLVMStructSetBody(
                self.0.as_ptr(),
                elements
                    .iter()
                    .map(|ty| ty.0.as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                elements.len() as u32,
                packed as i32
            )
        }
    }

    pub fn count_struct_element_types(&self) -> u32 {
        unsafe { LLVMCountStructElementTypes(self.0.as_ptr()) }
    }

    pub fn get_struct_element_types(&self) -> Vec<Type> {
        unsafe {
            let len = LLVMCountStructElementTypes(self.0.as_ptr()) as usize;
            let ptr: *mut *mut LLVMType =
                alloc(Layout::array::<*mut LLVMType>(len).unwrap()) as _;
            LLVMGetStructElementTypes(self.0.as_ptr(), ptr);

            std::slice::from_raw_parts_mut(ptr, len)
                .iter()
                .map(|&ty| Type::from_raw(NonNull::new_unchecked(ty)))
                .collect()
        }
    }

    pub fn struct_get_type_at_index(&self, index: u32) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMStructGetTypeAtIndex(self.0.as_ptr(), index)
            )
        })
    }

    pub fn is_packed_struct(&self) -> bool {
        unsafe { LLVMIsPackedStruct(self.0.as_ptr()) != 0 }
    }

    pub fn is_opaque_struct(&self) -> bool {
        unsafe { LLVMIsOpaqueStruct(self.0.as_ptr()) != 0 }
    }

    pub fn is_literal_struct(&self) -> bool {
        unsafe { LLVMIsLiteralStruct(self.0.as_ptr()) != 0 }
    }

    pub fn get_element_type(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMGetElementType(self.0.as_ptr()))
        })
    }

    /* TODO

    pub fn get_subtypes(&self) -> Vec<Type> {
        unsafe {
            let len = NEEDS LEN
            let ptr: *mut *mut LLVMType =
                alloc(Layout::array::<*mut LLVMType>(len).unwrap()) as _;
            LLVMGetSubtypes(self.0.as_ptr(), ptr);

            std::slice::from_raw_parts_mut(ptr, len)
                .iter()
                .map(|&ty| Type::from_raw(NonNull::new_unchecked(ty)))
                .collect()
        }
    }

     */

    pub fn get_num_contained_types(&self) -> u32 {
        unsafe { LLVMGetNumContainedTypes(self.0.as_ptr()) }
    }

    pub fn array_type(&self, element_count: u32) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMArrayType(self.0.as_ptr(), element_count))
        })
    }

    pub fn get_array_length(&self) -> u32 {
        unsafe { LLVMGetArrayLength(self.0.as_ptr()) }
    }

    pub fn pointer_type(&self, address_space: u32) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMPointerType(self.0.as_ptr(), address_space))
        })
    }

    pub fn get_pointer_address_space(&self) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(self.0.as_ptr()) }
    }

    pub fn vector_type(&self, element_count: u32) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMVectorType(self.0.as_ptr(), element_count))
        })
    }

    pub fn scalable_vector_type(&self, element_count: u32) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(LLVMScalableVectorType(self.0.as_ptr(), element_count))
        })
    }

    pub fn get_vector_size(&self) -> u32 {
        unsafe { LLVMGetVectorSize(self.0.as_ptr()) }
    }

    pub fn align_of(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMAlignOf(self.0.as_ptr())
            )
        })
    }

    pub fn size_of(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMSizeOf(self.0.as_ptr())
            )
        })
    }

    pub fn const_null(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstNull(self.0.as_ptr())
            )
        })
    }

    pub fn const_all_ones(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstAllOnes(self.0.as_ptr())
            )
        })
    }

    pub fn get_undef(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetUndef(self.0.as_ptr())
            )
        })
    }

    pub fn get_poison(&self) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetPoison(self.0.as_ptr())
            )
        })
    }

    pub fn is_null(value: Value) -> bool {
        unsafe { LLVMIsNull(value.as_raw().as_ptr()) != 0 }
    }

    pub fn const_int(&self, int: u64, sign_extend: bool) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstInt(self.0.as_ptr(), int, sign_extend as i32)
            )
        })
    }

    pub fn const_int_of_arbitrary_precision(&self, mut words: Vec<u64>) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstIntOfArbitraryPrecision(self.0.as_ptr(), words.len() as u32, words.as_mut_ptr())
            )
        })
    }

    pub fn const_int_of_string(&self, text: &str, radix: u8) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstIntOfString(
                    self.0.as_ptr(), to_c_string(Some(text)).as_ptr(), radix)
            )
        })
    }

    pub fn const_int_of_string_and_size(&self, text: &str, radix: u8) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstIntOfStringAndSize(
                    self.0.as_ptr(),
                    to_c_string(Some(text)).as_ptr(),
                    text.len() as u32,
                    radix
                )
            )
        })
    }

    pub fn const_float(&self, float: f64) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstReal(
                    self.0.as_ptr(), float
                )
            )
        })
    }

    pub fn const_float_of_string(&self, text: &str) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstRealOfStringAndSize(self.0.as_ptr(), to_c_string(Some(text)).as_ptr(), text.len() as u32)
            )
        })
    }

    pub fn const_array(&self, values: Vec<Value>) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstArray(
                    self.0.as_ptr(),
                    values.iter()
                        .map(|value| value.as_raw().as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    values.len() as u32
                )
            )
        })
    }

    pub fn const_named_struct(&self, values: Vec<Value>) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstNamedStruct(
                    self.0.as_ptr(),
                    values.iter()
                        .map(|value| value.as_raw().as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    values.len() as u32
                )
            )
        })
    }

    pub fn get_kind(&self) -> TypeKind {
        unsafe { transmute(LLVMGetTypeKind(self.0.as_ptr())) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TypeKind {
    // TODO: Implement
    Void = 0,
    F16 = 1,
    F32 = 2,
    F64 = 3,
    X86F80 = 4,
    BF16 = 18,
    F128 = 5,
    PpcF128 = 6,
    // TODO: Implement
    Label = 7,
    Int = 8,
    Fun = 9,
    Struct = 10,
    // TODO: Implement
    Array = 11,
    Ptr = 12,
    // TODO: Implement
    Vector = 13,
    // TODO: Implement
    Metadata = 14,
    // TODO: Implement
    X86Mmx = 15,
    // TODO: Implement
    Token = 16,
    // TODO: Implement
    ScalableVector = 17,
    // TODO: Implement
    X86Amx = 19,
}

impl TypeKind {
    #[inline(always)]
    pub fn of_llvm_type(llvm_type: &LlvmType) -> TypeKind {
        let kind = unsafe { LLVMGetTypeKind(llvm_type.as_raw().as_ptr()) };
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => TypeKind::Void,
            LLVMTypeKind::LLVMHalfTypeKind => TypeKind::F16,
            LLVMTypeKind::LLVMFloatTypeKind => TypeKind::F32,
            LLVMTypeKind::LLVMDoubleTypeKind => TypeKind::F64,
            LLVMTypeKind::LLVMX86_FP80TypeKind => TypeKind::X86F80,
            LLVMTypeKind::LLVMFP128TypeKind => TypeKind::F128,
            LLVMTypeKind::LLVMPPC_FP128TypeKind => TypeKind::PpcF128,
            LLVMTypeKind::LLVMLabelTypeKind => TypeKind::Label,
            LLVMTypeKind::LLVMIntegerTypeKind => TypeKind::Int,
            LLVMTypeKind::LLVMFunctionTypeKind => TypeKind::Fun,
            LLVMTypeKind::LLVMStructTypeKind => TypeKind::Struct,
            LLVMTypeKind::LLVMArrayTypeKind => TypeKind::Array,
            LLVMTypeKind::LLVMPointerTypeKind => TypeKind::Ptr,
            LLVMTypeKind::LLVMVectorTypeKind => TypeKind::Vector,
            LLVMTypeKind::LLVMMetadataTypeKind => TypeKind::Metadata,
            LLVMTypeKind::LLVMX86_MMXTypeKind => TypeKind::X86Mmx,
            LLVMTypeKind::LLVMTokenTypeKind => TypeKind::Token,
            LLVMTypeKind::LLVMScalableVectorTypeKind => TypeKind::ScalableVector,
            LLVMTypeKind::LLVMBFloatTypeKind => TypeKind::BF16,
            LLVMTypeKind::LLVMX86_AMXTypeKind => TypeKind::X86Amx,
        }
    }
}