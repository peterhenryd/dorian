use crate::llvm::sys::core::{LLVMConstInt, LLVMConstIntOfStringAndSize, LLVMGetTypeKind, LLVMPointerType, LLVMGetElementType, LLVMArrayType};
use crate::llvm::sys::{LLVMType, LLVMTypeKind};
use crate::llvm::value::Value;
use crate::llvm::{to_c_string, AddressSpace};
use std::mem::transmute;
use std::ptr::NonNull;
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

    pub fn LLVMGetInlineAsm(
        Ty: LLVMTypeRef,
        AsmString: *mut ::libc::c_char,
        AsmStringSize: ::libc::size_t,
        Constraints: *mut ::libc::c_char,
        ConstraintsSize: ::libc::size_t,
        HasSideEffects: LLVMBool,
        IsAlignStack: LLVMBool,
        Dialect: LLVMInlineAsmDialect,
        CanThrow: LLVMBool,
    ) -> LLVMValueRef;

    pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
    pub fn LLVMTypeIsSized(Ty: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMGetTypeContext(Ty: LLVMTypeRef) -> LLVMContextRef;
    pub fn LLVMDumpType(Val: LLVMTypeRef);
    pub fn LLVMPrintTypeToString(Val: LLVMTypeRef) -> *mut ::libc::c_char;

    pub fn LLVMGetIntTypeWidth(IntegerTy: LLVMTypeRef) -> ::libc::c_uint;


    pub fn LLVMGetStructName(Ty: LLVMTypeRef) -> *const ::libc::c_char;
    pub fn LLVMStructSetBody(
        StructTy: LLVMTypeRef,
        ElementTypes: *mut LLVMTypeRef,
        ElementCount: ::libc::c_uint,
        Packed: LLVMBool,
    );
    pub fn LLVMCountStructElementTypes(StructTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMGetStructElementTypes(StructTy: LLVMTypeRef, Dest: *mut LLVMTypeRef);
    /// Get the type of the element at the given index in a structure.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMStructGetTypeAtIndex(StructTy: LLVMTypeRef, i: ::libc::c_uint) -> LLVMTypeRef;
    /// Determine whether a structure is packed.
    pub fn LLVMIsPackedStruct(StructTy: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMIsOpaqueStruct(StructTy: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMIsLiteralStruct(StructTy: LLVMTypeRef) -> LLVMBool;

    // Core->Types->Sequential
    pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;
    /// Get the subtypes of the given type.
    pub fn LLVMGetSubtypes(Tp: LLVMTypeRef, Arr: *mut LLVMTypeRef);
    /// Return the number of types in the derived type.
    pub fn LLVMGetNumContainedTypes(Tp: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMArrayType(ElementType: LLVMTypeRef, ElementCount: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetArrayLength(ArrayTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMPointerType(ElementType: LLVMTypeRef, AddressSpace: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetPointerAddressSpace(PointerTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMVectorType(ElementType: LLVMTypeRef, ElementCount: ::libc::c_uint) -> LLVMTypeRef;
    /// Create a vector type that contains a defined type and has a scalable
    /// number of elements.
    ///
    /// The created type will exist in the context that its element type
    /// exists in.
    pub fn LLVMScalableVectorType(
        ElementType: LLVMTypeRef,
        ElementCount: ::libc::c_uint,
    ) -> LLVMTypeRef;
    /// Obtain the (possibly scalable) number of elements in a vector type.
    pub fn LLVMGetVectorSize(VectorTy: LLVMTypeRef) -> ::libc::c_uint;


    pub fn LLVMConstNull(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstAllOnes(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMGetUndef(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMGetPoison(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMIsNull(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMConstPointerNull(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstInt(
        IntTy: LLVMTypeRef,
        N: ::libc::c_ulonglong,
        SignExtend: LLVMBool,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntOfArbitraryPrecision(
        IntTy: LLVMTypeRef,
        NumWords: ::libc::c_uint,
        Words: *const u64,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntOfString(
        IntTy: LLVMTypeRef,
        Text: *const ::libc::c_char,
        Radix: u8,
    ) -> LLVMValueRef;
    pub fn LLVMConstIntOfStringAndSize(
        IntTy: LLVMTypeRef,
        Text: *const ::libc::c_char,
        SLen: ::libc::c_uint,
        Radix: u8,
    ) -> LLVMValueRef;
    pub fn LLVMConstReal(RealTy: LLVMTypeRef, N: ::libc::c_double) -> LLVMValueRef;
    pub fn LLVMConstRealOfString(RealTy: LLVMTypeRef, Text: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMConstRealOfStringAndSize(
        RealTy: LLVMTypeRef,
        Text: *const ::libc::c_char,
        SLen: ::libc::c_uint,
    ) -> LLVMValueRef;

}

impl Type {
    pub fn get_kind(&self) -> TypeKind {
        unsafe { transmute(LLVMGetTypeKind(self.0.as_ptr())) }
    }

    pub unsafe fn const_int(&self, int: u64, sign_extend: bool) -> Value {
        Value::from_raw(NonNull::new_unchecked(LLVMConstInt(
            self.0.as_ptr(),
            int,
            sign_extend as i32,
        )))
    }

    pub unsafe fn const_int_from_str(&self, str: &str, radix: u8) -> Value {
        Value::from_raw(NonNull::new_unchecked(LLVMConstIntOfStringAndSize(
            self.0.as_ptr(),
            to_c_string(Some(str)).as_ptr(),
            (str.len() + 1) as u32,
            radix,
        )))
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