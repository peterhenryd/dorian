use crate::llvm::sys::target::{LLVMOpaqueTargetData, LLVMPointerSize, LLVMDisposeTargetData};
use crate::llvm::sys::target_machine::{LLVMGetTargetFromTriple, LLVMTarget};
use crate::llvm::target::triple::TargetTriple;
use crate::llvm::{CodeModel, from_c_string, OptimizationLevel, RelocMode, to_c_string};
use std::mem::{MaybeUninit, transmute};
use std::ptr::NonNull;
use llvm_sys::target::*;
use llvm_sys::target_machine::{LLVMCreateTargetMachine, LLVMGetTargetDescription, LLVMGetTargetName, LLVMTargetHasAsmBackend, LLVMTargetHasJIT, LLVMTargetHasTargetMachine};
use crate::llvm::target::machine::TargetMachine;
use crate::llvm::types::Type;
use crate::types::LlvmType;
use crate::llvm::value::Value;

pub mod init;
pub mod machine;
pub mod triple;

pub struct Target<'a>(NonNull<LLVMTarget>, TargetTriple<'a>);

impl<'a> Target<'a> {
    pub fn from_raw(ptr: NonNull<LLVMTarget>, target_triple: TargetTriple<'a>) -> Target<'a> {
        Target(ptr, target_triple)
    }

    pub fn as_raw(&self) -> NonNull<LLVMTarget> {
        self.0
    }

    pub fn from_triple(target_triple: TargetTriple<'a>) -> Result<Target<'a>, &str> {
        unsafe {
            let mut triple = MaybeUninit::uninit();
            let mut error_message = MaybeUninit::uninit();

            let res = LLVMGetTargetFromTriple(
                to_c_string(Some(target_triple.as_str())).as_ptr(),
                triple.as_mut_ptr(),
                error_message.as_mut_ptr(),
            );

            if res == 0 {
                Err(from_c_string(error_message.assume_init()))
            } else {
                Ok(Target(NonNull::new_unchecked(triple.assume_init()), target_triple))
            }
        }
    }

    // pub fn LLVMGetFirstTarget() -> LLVMTargetRef;
    // pub fn LLVMGetNextTarget(T: LLVMTargetRef) -> LLVMTargetRef;
    // pub fn LLVMGetTargetFromName(Name: *const ::libc::c_char) -> LLVMTargetRef;

    pub fn get_name(&self) -> &str {
        unsafe { from_c_string(LLVMGetTargetName(self.0.as_ptr())) }
    }

    pub fn get_description(&self) -> &str {
        unsafe { from_c_string(LLVMGetTargetDescription(self.0.as_ptr())) }
    }

    pub fn has_jit(&self) -> bool {
        unsafe { LLVMTargetHasJIT(self.0.as_ptr()) != 0 }
    }

    pub fn has_target_machine(&self) -> bool {
        unsafe { LLVMTargetHasTargetMachine(self.0.as_ptr()) != 0 }
    }

    pub fn has_asm_backend(&self) -> bool {
        unsafe { LLVMTargetHasAsmBackend(self.0.as_ptr()) != 0 }
    }

    pub fn create_machine(
        &self,
        cpu: &str,
        features: &str,
        optimization_level: OptimizationLevel,
        reloc_mode: RelocMode,
        code_model: CodeModel,
    ) -> TargetMachine {
        TargetMachine::from_raw(unsafe {
            NonNull::new_unchecked(LLVMCreateTargetMachine(
                self.0.as_ptr(),
                to_c_string(Some(self.1.as_str())).as_ptr(),
                to_c_string(Some(cpu)).as_ptr(),
                to_c_string(Some(features)).as_ptr(),
                transmute(optimization_level),
                transmute(reloc_mode),
                transmute(code_model),
            ))
        })
    }
}

#[derive(Debug)]
pub struct TargetData(NonNull<LLVMOpaqueTargetData>);

impl TargetData {
    pub fn from_raw(ptr: NonNull<LLVMOpaqueTargetData>) -> TargetData {
        TargetData(ptr)
    }

    pub fn as_raw(&self) -> NonNull<LLVMOpaqueTargetData> {
        self.0
    }

    pub fn get_ptr_size(&self) -> u32 {
        unsafe {
            LLVMPointerSize(self.0.as_ptr())
        }
    }

    pub fn copy_string_rep_of_target_data(&self) -> &str {
        from_c_string(unsafe {
            LLVMCopyStringRepOfTargetData(self.0.as_ptr())
        })
    }

    pub fn byte_order(&self) -> ByteOrdering {
        unsafe {
            transmute(LLVMByteOrder(self.0.as_ptr()))
        }
    }

    pub fn pointer_size(&self) -> u32 {
        unsafe {
            LLVMPointerSize(self.0.as_ptr())
        }
    }

    pub fn pointer_size_for_as(&self, r#as: u32) -> u32 {
        unsafe {
            LLVMPointerSizeForAS(self.0.as_ptr(), r#as)
        }
    }

    pub fn int_ptr_type(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMIntPtrType(self.0.as_ptr())
            )
        })
    }

    pub fn int_ptr_type_for_as(&self, r#as: u32) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMIntPtrTypeForAS(self.0.as_ptr(), r#as)
            )
        })
    }

    pub fn size_of_type_in_bits(&self, ty: LlvmType) -> u64 {
        unsafe {
            LLVMSizeOfTypeInBits(self.0.as_ptr(), ty.as_raw().as_ptr())
        }
    }

    pub fn store_size_of_type(&self, ty: LlvmType) -> u64 {
        unsafe {
            LLVMSizeOfTypeInBits(self.0.as_ptr(), ty.as_raw().as_ptr())
        }
    }

    pub fn abi_size_of_type(&self, ty: LlvmType) -> u64 {
        unsafe {
            LLVMSizeOfTypeInBits(self.0.as_ptr(), ty.as_raw().as_ptr())
        }
    }

    pub fn abi_alignment_of_type(&self, ty: LlvmType) -> u32 {
        unsafe {
            LLVMABIAlignmentOfType(self.0.as_ptr(), ty.as_raw().as_ptr())
        }
    }

    pub fn call_frame_alignment_of_type(&self, ty: LlvmType) -> u32 {
        unsafe {
            LLVMCallFrameAlignmentOfType(self.0.as_ptr(), ty.as_raw().as_ptr())
        }
    }

    pub fn preferred_alignment_of_type(&self, ty: LlvmType) -> u32 {
        unsafe {
            LLVMPreferredAlignmentOfType(self.0.as_ptr(), ty.as_raw().as_ptr())
        }
    }

    pub fn preferred_alignment_of_global(&self, global_var: Value) -> u32 {
        unsafe {
            LLVMPreferredAlignmentOfGlobal(self.0.as_ptr(), global_var.as_raw().as_ptr())
        }
    }

    pub fn element_at_offset(&self, struct_type: Type, offset: u64) -> u32 {
        unsafe {
            LLVMElementAtOffset(self.0.as_ptr(), struct_type.as_raw().as_ptr(), offset)
        }
    }

    pub fn offset_of_element(&self, struct_type: Type, element: u32) -> u64 {
        unsafe {
            LLVMOffsetOfElement(self.0.as_ptr(), struct_type.as_raw().as_ptr(), element)
        }
    }
}

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetData(self.0.as_ptr());
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ByteOrdering {
    BigEndian = 0,
    LittleEndian = 1,
}

pub struct TargetLibraryInfotData(NonNull<LLVMOpaqueTargetLibraryInfotData>);

impl TargetLibraryInfotData {
    pub fn from_raw(raw: NonNull<LLVMOpaqueTargetLibraryInfotData>) -> TargetLibraryInfotData {
        TargetLibraryInfotData(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMOpaqueTargetLibraryInfotData> {
        self.0
    }
}

pub struct TargetInit;

impl TargetInit {
    pub fn amd_gpu_target_info() {
        unsafe { LLVMInitializeAMDGPUTargetInfo() }
    }

    pub fn amd_gpu_target() {
        unsafe { LLVMInitializeAMDGPUTarget() }
    }

    pub fn amd_gpu_target_mc() {
        unsafe { LLVMInitializeAMDGPUTargetMC() }
    }

    pub fn amd_gpu_asm_printer() {
        unsafe { LLVMInitializeAMDGPUAsmPrinter() }
    }

    pub fn amd_gpu_asm_parser() {
        unsafe { LLVMInitializeAMDGPUAsmParser() }
    }

    pub fn system_z_target_info() {
        unsafe { LLVMInitializeSystemZTargetInfo() }
    }

    pub fn system_z_target() {
        unsafe { LLVMInitializeSystemZTarget() }
    }

    pub fn system_z_target_mc() {
        unsafe { LLVMInitializeSystemZTargetMC() }
    }

    pub fn system_z_asm_printer() {
        unsafe { LLVMInitializeSystemZAsmPrinter() }
    }

    pub fn system_z_asm_parser() {
        unsafe { LLVMInitializeSystemZAsmParser() }
    }

    pub fn system_z_disassembler() {
        unsafe { LLVMInitializeSystemZDisassembler() }
    }

    pub fn hexagon_target_info() {
        unsafe { LLVMInitializeHexagonTargetInfo() }
    }

    pub fn hexagon_target() {
        unsafe { LLVMInitializeHexagonTarget() }
    }

    pub fn hexagon_target_mc() {
        unsafe { LLVMInitializeHexagonTargetMC() }
    }

    pub fn hexagon_asm_printer() {
        unsafe { LLVMInitializeHexagonAsmPrinter() }
    }

    pub fn hexagon_disassembler() {
        unsafe { LLVMInitializeHexagonDisassembler() }
    }

    pub fn nvptx_target_info() {
        unsafe { LLVMInitializeNVPTXTargetInfo() }
    }

    pub fn nvptx_target() {
        unsafe { LLVMInitializeNVPTXTarget() }
    }

    pub fn nvptx_target_mc() {
        unsafe { LLVMInitializeNVPTXTargetMC() }
    }

    pub fn nvptx_asm_printer() {
        unsafe { LLVMInitializeNVPTXAsmPrinter() }
    }

    pub fn msp430_target_info() {
        unsafe { LLVMInitializeMSP430TargetInfo() }
    }

    pub fn msp430_target() {
        unsafe { LLVMInitializeMSP430Target() }
    }

    pub fn msp430_target_mc() {
        unsafe { LLVMInitializeMSP430TargetMC() }
    }

    pub fn msp430_asm_printer() {
        unsafe { LLVMInitializeMSP430AsmPrinter() }
    }

    pub fn xcore_target_info() {
        unsafe { LLVMInitializeXCoreTargetInfo() }
    }

    pub fn xcore_target() {
        unsafe { LLVMInitializeXCoreTarget() }
    }

    pub fn xcore_target_mc() {
        unsafe { LLVMInitializeXCoreTargetMC() }
    }

    pub fn xcore_asm_printer() {
        unsafe { LLVMInitializeXCoreAsmPrinter() }
    }

    pub fn xcore_disassembler() {
        unsafe { LLVMInitializeXCoreDisassembler() }
    }

    pub fn mips_target_info() {
        unsafe { LLVMInitializeMipsTargetInfo() }
    }

    pub fn mips_target() {
        unsafe { LLVMInitializeMipsTarget() }
    }

    pub fn mips_target_mc() {
        unsafe { LLVMInitializeMipsTargetMC() }
    }

    pub fn mips_asm_printer() {
        unsafe { LLVMInitializeMipsAsmPrinter() }
    }

    pub fn mips_asm_parser() {
        unsafe { LLVMInitializeMipsAsmParser() }
    }

    pub fn mips_disassembler() {
        unsafe { LLVMInitializeMipsDisassembler() }
    }

    pub fn aarch64_target_info() {
        unsafe { LLVMInitializeAArch64TargetInfo() }
    }

    pub fn aarch64_target() {
        unsafe { LLVMInitializeAArch64Target() }
    }

    pub fn aarch64_target_mc() {
        unsafe { LLVMInitializeAArch64TargetMC() }
    }

    pub fn aarch64_asm_printer() {
        unsafe { LLVMInitializeAArch64AsmPrinter() }
    }

    pub fn aarch64_asm_parser() {
        unsafe { LLVMInitializeAArch64AsmParser() }
    }

    pub fn aarch64_disassembler() {
        unsafe { LLVMInitializeAArch64Disassembler() }
    }

    pub fn arm_target_info() {
        unsafe { LLVMInitializeARMTargetInfo() }
    }

    pub fn arm_target() {
        unsafe { LLVMInitializeARMTarget() }
    }

    pub fn arm_target_mc() {
        unsafe { LLVMInitializeARMTargetMC() }
    }

    pub fn arm_asm_printer() {
        unsafe { LLVMInitializeARMAsmPrinter() }
    }

    pub fn arm_asm_parser() {
        unsafe { LLVMInitializeARMAsmParser() }
    }

    pub fn arm_disassembler() {
        unsafe { LLVMInitializeARMDisassembler() }
    }

    pub fn powerpc_target_info() {
        unsafe { LLVMInitializePowerPCTargetInfo() }
    }

    pub fn powerpc_target() {
        unsafe { LLVMInitializePowerPCTarget() }
    }

    pub fn powerpc_target_mc() {
        unsafe { LLVMInitializePowerPCTargetMC() }
    }

    pub fn powerpc_asm_printer() {
        unsafe { LLVMInitializePowerPCAsmPrinter() }
    }

    pub fn powerpc_asm_parser() {
        unsafe { LLVMInitializePowerPCAsmParser() }
    }

    pub fn powerpc_disassembler() {
        unsafe { LLVMInitializePowerPCDisassembler() }
    }

    pub fn sparc_target_info() {
        unsafe { LLVMInitializeSparcTargetInfo() }
    }

    pub fn sparc_target() {
        unsafe { LLVMInitializeSparcTarget() }
    }

    pub fn sparc_target_mc() {
        unsafe { LLVMInitializeSparcTargetMC() }
    }

    pub fn sparc_asm_printer() {
        unsafe { LLVMInitializeSparcAsmPrinter() }
    }

    pub fn sparc_asm_parser() {
        unsafe { LLVMInitializeSparcAsmParser() }
    }

    pub fn sparc_disassembler() {
        unsafe { LLVMInitializeSparcDisassembler() }
    }

    pub fn x86_target_info() {
        unsafe { LLVMInitializeX86TargetInfo() }
    }

    pub fn x86_target() {
        unsafe { LLVMInitializeX86Target() }
    }

    pub fn x86_target_mc() {
        unsafe { LLVMInitializeX86TargetMC() }
    }

    pub fn x86_asm_printer() {
        unsafe { LLVMInitializeX86AsmPrinter() }
    }

    pub fn x86_asm_parser() {
        unsafe { LLVMInitializeX86AsmParser() }
    }

    pub fn x86_disassembler() {
        unsafe { LLVMInitializeX86Disassembler() }
    }

    pub fn bpf_target_info() {
        unsafe { LLVMInitializeBPFTargetInfo() }
    }

    pub fn bpf_target() {
        unsafe { LLVMInitializeBPFTarget() }
    }

    pub fn bpf_target_mc() {
        unsafe { LLVMInitializeBPFTargetMC() }
    }

    pub fn bpf_asm_printer() {
        unsafe { LLVMInitializeBPFAsmPrinter() }
    }

    pub fn bpf_disassembler() {
        unsafe { LLVMInitializeBPFDisassembler() }
    }

    pub fn lanai_target_info() {
        unsafe { LLVMInitializeLanaiTargetInfo() }
    }

    pub fn lanai_target() {
        unsafe { LLVMInitializeLanaiTarget() }
    }

    pub fn lanai_target_mc() {
        unsafe { LLVMInitializeLanaiTargetMC() }
    }

    pub fn lanai_asm_printer() {
        unsafe { LLVMInitializeLanaiAsmPrinter() }
    }

    pub fn lanai_asm_parser() {
        unsafe { LLVMInitializeLanaiAsmParser() }
    }

    pub fn lanai_disassembler() {
        unsafe { LLVMInitializeLanaiDisassembler() }
    }

    pub fn riscv_target_info() {
        unsafe { LLVMInitializeRISCVTargetInfo() }
    }

    pub fn riscv_target() {
        unsafe { LLVMInitializeRISCVTarget() }
    }

    pub fn riscv_target_mc() {
        unsafe { LLVMInitializeRISCVTargetMC() }
    }

    pub fn riscv_asm_printer() {
        unsafe { LLVMInitializeRISCVAsmPrinter() }
    }

    pub fn riscv_asm_parser() {
        unsafe { LLVMInitializeRISCVAsmParser() }
    }

    pub fn riscv_disassembler() {
        unsafe { LLVMInitializeRISCVDisassembler() }
    }

    pub fn web_assembly_target_info() {
        unsafe { LLVMInitializeWebAssemblyTargetInfo() }
    }

    pub fn web_assembly_target() {
        unsafe { LLVMInitializeWebAssemblyTarget() }
    }

    pub fn web_assembly_target_mc() {
        unsafe { LLVMInitializeWebAssemblyTargetMC() }
    }

    pub fn web_assembly_asm_printer() {
        unsafe { LLVMInitializeWebAssemblyAsmPrinter() }
    }

    pub fn web_assembly_asm_parser() {
        unsafe { LLVMInitializeWebAssemblyAsmParser() }
    }

    pub fn web_assembly_disassembler() {
        unsafe { LLVMInitializeWebAssemblyDisassembler() }
    }


    pub fn all_target_infos() {
        unsafe { LLVM_InitializeAllTargetInfos() }
    }

    pub fn all_targets() {
        unsafe { LLVM_InitializeAllTargets() }
    }

    pub fn all_target_mcs() {
        unsafe { LLVM_InitializeAllTargetMCs() }
    }

    pub fn all_asm_printers() {
        unsafe { LLVM_InitializeAllAsmPrinters() }
    }

    pub fn all_asm_parsers() {
        unsafe { LLVM_InitializeAllAsmParsers() }
    }

    pub fn all_disassemblers() {
        unsafe { LLVM_InitializeAllDisassemblers() }
    }

    pub fn initialize_native_target() -> bool {
        unsafe { LLVM_InitializeNativeTarget() != 0 }
    }

    pub fn initialize_native_asm_parser() -> bool {
        unsafe { LLVM_InitializeNativeAsmParser() != 0 }
    }

    pub fn native_asm_printer() -> bool {
        unsafe { LLVM_InitializeNativeAsmPrinter() != 0 }
    }

    pub fn native_disassembler() -> bool {
        unsafe { LLVM_InitializeNativeDisassembler() != 0 }
    }
}

/* TODO: target data
extern "C" {
    /// Create target data from a target layout string.
    pub fn LLVMCreateTargetData(StringRep: *const ::libc::c_char) -> LLVMTargetDataRef;
    pub fn LLVMAddTargetLibraryInfo(TLI: LLVMTargetLibraryInfoRef, PM: LLVMPassManagerRef);
}

 */