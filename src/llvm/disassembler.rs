use std::mem::MaybeUninit;
use std::ptr::NonNull;
use llvm_sys::disassembler::{LLVMDisasmDispose, LLVMDisasmInstruction, LLVMOpaqueDisasmContext, LLVMSetDisasmOptions};
use crate::llvm::to_c_string;

#[derive(Debug)]
pub struct DisasmContext(NonNull<LLVMOpaqueDisasmContext>);

impl DisasmContext {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMOpaqueDisasmContext>) -> DisasmContext {
        DisasmContext(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMOpaqueDisasmContext> {
        self.0
    }

    pub fn create(
        triple_name: &str,
        dis_info: *mut ::libc::c_void,
        tag_type: ::libc::c_int,
        get_op_info: LLVMOpInfoCallback,
        symbol_look_up: LLVMSymbolLookupCallback,
    ) -> DisasmContext;

    pub fn create_cpu(
        triple: *const ::libc::c_char,
        cpu: *const ::libc::c_char,
        dis_info: *mut ::libc::c_void,
        tag_type: ::libc::c_int,
        get_op_info: LLVMOpInfoCallback,
        symbol_look_up: LLVMSymbolLookupCallback,
    ) -> DisasmContext;

    pub fn create_cpu_features(
        Triple: *const ::libc::c_char,
        CPU: *const ::libc::c_char,
        Features: *const ::libc::c_char,
        DisInfo: *mut ::libc::c_void,
        TagType: ::libc::c_int,
        GetOpInfo: LLVMOpInfoCallback,
        SymbolLookUp: LLVMSymbolLookupCallback,
    ) -> DisasmContext {

    }

    pub fn set_options(&self, options: u64) -> i32 {
        unsafe {
            LLVMSetDisasmOptions(
                self.0.as_ptr(), options
            )
        }
    }

    pub fn instruction(
        &self,
        mut bytes: Vec<u8>,
        pc: u64,
        out: &str,
    ) -> usize {
        unsafe {
            LLVMDisasmInstruction(
                self.0.as_ptr(),
                bytes.as_mut_ptr(),
                bytes.len() as u64,
                pc,
                to_c_string(Some(out)).into_raw(),
                out.len(),
            )
        }
    }
}

impl Drop for DisasmContext {
    fn drop(&mut self) {
        unsafe {
            LLVMDisasmDispose(self.0.as_ptr());
        }
    }
}

pub type LLVMOpInfoCallback = Option<
    extern "C" fn(
        DisInfo: *mut ::libc::c_void,
        PC: u64,
        Offset: u64,
        Size: u64,
        TagType: ::libc::c_int,
        TagBuf: *mut ::libc::c_void,
    ) -> ::libc::c_int,
>;

#[repr(C)]
#[derive(Debug)]
pub struct LLVMOpInfoSymbol1 {
    /// 1 if this symbol is present.
    pub Present: u64,
    /// Symbol name if not NULL.
    pub Name: *const ::libc::c_char,
    /// Symbol value if name is NULL.
    pub Value: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct Struct_LLVMOpInfo1 {
    pub AddSymbol: LLVMOpInfoSymbol1,
    pub SubtractSymbol: LLVMOpInfoSymbol1,
    pub Value: u64,
    pub VariantKind: u64,
}

pub const LLVMDisassembler_VariantKind_None: u64 = 0;
pub const LLVMDisassembler_VariantKind_ARM_HI16: u64 = 1;
pub const LLVMDisassembler_VariantKind_ARM_LO16: u64 = 2;
pub const LLVMDisassembler_VariantKind_ARM64_PAGE: u64 = 1;
pub const LLVMDisassembler_VariantKind_ARM64_PAGEOFF: u64 = 2;
pub const LLVMDisassembler_VariantKind_ARM64_GOTPAGE: u64 = 3;
pub const LLVMDisassembler_VariantKind_ARM64_GOTPAGEOFF: u64 = 4;
pub const LLVMDisassembler_VariantKind_ARM64_TLVP: u64 = 5;
pub const LLVMDisassembler_VariantKind_ARM64_TLVOFF: u64 = 6;

pub const LLVMDisassembler_ReferenceType_InOut_None: u64 = 0;
pub const LLVMDisassembler_ReferenceType_In_Branch: u64 = 1;
pub const LLVMDisassembler_ReferenceType_In_PCrel_Load: u64 = 2;
pub const LLVMDisassembler_ReferenceType_In_ARM64_ADRP: u64 = 0x100000001;
pub const LLVMDisassembler_ReferenceType_In_ARM64_ADDXri: u64 = 0x100000002;
pub const LLVMDisassembler_ReferenceType_In_ARM64_LDRXui: u64 = 0x100000003;
pub const LLVMDisassembler_ReferenceType_In_ARM64_LDRXl: u64 = 0x100000004;
pub const LLVMDisassembler_ReferenceType_In_ARM64_ADR: u64 = 0x100000005;

pub const LLVMDisassembler_ReferenceType_Out_SymbolStub: u64 = 1;
pub const LLVMDisassembler_ReferenceType_Out_LitPool_SymAddr: u64 = 2;
pub const LLVMDisassembler_ReferenceType_Out_LitPool_CstrAddr: u64 = 3;

pub const LLVMDisassembler_ReferenceType_Out_Objc_CFString_Ref: u64 = 4;
pub const LLVMDisassembler_ReferenceType_Out_Objc_Message: u64 = 5;
pub const LLVMDisassembler_ReferenceType_Out_Objc_Message_Ref: u64 = 6;
pub const LLVMDisassembler_ReferenceType_Out_Objc_Selector_Ref: u64 = 7;
pub const LLVMDisassembler_ReferenceType_Out_Objc_Class_Ref: u64 = 8;
pub const LLVMDisassembler_ReferenceType_DeMangled_Name: u64 = 9;

pub const LLVMDisassembler_Option_UseMarkup: u64 = 1;
pub const LLVMDisassembler_Option_PrintImmHex: u64 = 2;
pub const LLVMDisassembler_Option_AsmPrinterVariant: u64 = 4;
pub const LLVMDisassembler_Option_SetInstrComments: u64 = 8;
pub const LLVMDisassembler_Option_PrintLatency: u64 = 16;

pub type LLVMSymbolLookupCallback = Option<
    extern "C" fn(
        DisInfo: *mut ::libc::c_void,
        ReferenceValue: u64,
        ReferenceType: *mut u64,
        ReferencePC: u64,
        ReferenceName: *mut *const ::libc::c_char,
    ) -> *const ::libc::c_char,
>;
