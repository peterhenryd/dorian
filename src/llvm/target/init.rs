use crate::llvm::sys::target::*;
use crate::llvm::target::Target;

impl Target {
    pub fn initialize_amd_gpu_target_info() {
        unsafe {
            LLVMInitializeAMDGPUTargetInfo();
        }
    }

    pub fn initialize_amd_gpu_target() {
        unsafe {
            LLVMInitializeAMDGPUTarget();
        }
    }

    pub fn initialize_amd_gpu_target_mc() {
        unsafe {
            LLVMInitializeAMDGPUTargetMC();
        }
    }

    pub fn initialize_amd_gpu_asm_printer() {
        unsafe {
            LLVMInitializeAMDGPUAsmPrinter();
        }
    }

    pub fn initialize_amd_gpu_asm_parser() {
        unsafe {
            LLVMInitializeAMDGPUAsmParser();
        }
    }

    pub fn initialize_system_z_target_info() {
        unsafe {
            LLVMInitializeSystemZTargetInfo();
        }
    }

    pub fn initialize_system_z_target() {
        unsafe {
            LLVMInitializeSystemZTarget();
        }
    }

    pub fn initialize_system_z_target_mc() {
        unsafe {
            LLVMInitializeSystemZTargetMC();
        }
    }

    pub fn initialize_system_z_asm_printer() {
        unsafe {
            LLVMInitializeSystemZAsmPrinter();
        }
    }

    pub fn initialize_system_z_asm_parser() {
        unsafe {
            LLVMInitializeSystemZAsmParser();
        }
    }

    pub fn initialize_system_z_disassembler() {
        unsafe {
            LLVMInitializeSystemZDisassembler();
        }
    }

    pub fn initialize_hexagon_target_info() {
        unsafe {
            LLVMInitializeHexagonTargetInfo();
        }
    }

    pub fn initialize_hexagon_target() {
        unsafe {
            LLVMInitializeHexagonTarget();
        }
    }

    pub fn initialize_hexagon_target_mc() {
        unsafe {
            LLVMInitializeHexagonTargetMC();
        }
    }

    pub fn initialize_hexagon_asm_printer() {
        unsafe {
            LLVMInitializeHexagonAsmPrinter();
        }
    }

    pub fn initialize_hexagon_disassembler() {
        unsafe {
            LLVMInitializeHexagonDisassembler();
        }
    }

    pub fn initialize_nvptx_target_info() {
        unsafe {
            LLVMInitializeNVPTXTargetInfo();
        }
    }

    pub fn initialize_nvptx_target() {
        unsafe {
            LLVMInitializeNVPTXTarget();
        }
    }

    pub fn initialize_nvptx_target_mc() {
        unsafe {
            LLVMInitializeNVPTXTargetMC();
        }
    }

    pub fn initialize_nvptx_asm_printer() {
        unsafe {
            LLVMInitializeNVPTXAsmPrinter();
        }
    }

    pub fn initialize_msp430_target_info() {
        unsafe {
            LLVMInitializeMSP430TargetInfo();
        }
    }

    pub fn initialize_msp430_target() {
        unsafe {
            LLVMInitializeMSP430Target();
        }
    }

    pub fn initialize_msp430_target_mc() {
        unsafe {
            LLVMInitializeMSP430TargetMC();
        }
    }

    pub fn initialize_msp430_asm_printer() {
        unsafe {
            LLVMInitializeMSP430AsmPrinter();
        }
    }

    pub fn initialize_xcore_target_info() {
        unsafe {
            LLVMInitializeXCoreTargetInfo();
        }
    }

    pub fn initialize_xcore_target() {
        unsafe {
            LLVMInitializeXCoreTarget();
        }
    }

    pub fn initialize_xcore_target_mc() {
        unsafe {
            LLVMInitializeXCoreTargetMC();
        }
    }

    pub fn initialize_xcore_asm_printer() {
        unsafe {
            LLVMInitializeXCoreAsmPrinter();
        }
    }

    pub fn initialize_xcore_disassembler() {
        unsafe {
            LLVMInitializeXCoreDisassembler();
        }
    }

    pub fn initialize_mips_target_info() {
        unsafe {
            LLVMInitializeMipsTargetInfo();
        }
    }

    pub fn initialize_mips_target() {
        unsafe {
            LLVMInitializeMipsTarget();
        }
    }

    pub fn initialize_mips_target_mc() {
        unsafe {
            LLVMInitializeMipsTargetMC();
        }
    }

    pub fn initialize_mips_asm_printer() {
        unsafe {
            LLVMInitializeMipsAsmPrinter();
        }
    }

    pub fn initialize_mips_asm_parser() {
        unsafe {
            LLVMInitializeMipsAsmParser();
        }
    }

    pub fn initialize_mips_disassembler() {
        unsafe {
            LLVMInitializeMipsDisassembler();
        }
    }

    pub fn initialize_aarch64_target_info() {
        unsafe {
            LLVMInitializeAArch64TargetInfo();
        }
    }

    pub fn initialize_aarch64_target() {
        unsafe {
            LLVMInitializeAArch64Target();
        }
    }

    pub fn initialize_aarch64_target_mc() {
        unsafe {
            LLVMInitializeAArch64TargetMC();
        }
    }

    pub fn initialize_aarch64_asm_printer() {
        unsafe {
            LLVMInitializeAArch64AsmPrinter();
        }
    }

    pub fn initialize_aarch64_asm_parser() {
        unsafe {
            LLVMInitializeAArch64AsmParser();
        }
    }

    pub fn initialize_aarch64_disassembler() {
        unsafe {
            LLVMInitializeAArch64Disassembler();
        }
    }

    pub fn initialize_arm_target_info() {
        unsafe {
            LLVMInitializeARMTargetInfo();
        }
    }

    pub fn initialize_arm_target() {
        unsafe {
            LLVMInitializeARMTarget();
        }
    }

    pub fn initialize_arm_target_mc() {
        unsafe {
            LLVMInitializeARMTargetMC();
        }
    }

    pub fn initialize_arm_asm_printer() {
        unsafe {
            LLVMInitializeARMAsmPrinter();
        }
    }

    pub fn initialize_arm_asm_parser() {
        unsafe {
            LLVMInitializeARMAsmParser();
        }
    }

    pub fn initialize_arm_disassembler() {
        unsafe {
            LLVMInitializeARMDisassembler();
        }
    }

    pub fn initialize_powerpc_target_info() {
        unsafe {
            LLVMInitializePowerPCTargetInfo();
        }
    }

    pub fn initialize_powerpc_target() {
        unsafe {
            LLVMInitializePowerPCTarget();
        }
    }

    pub fn initialize_powerpc_target_mc() {
        unsafe {
            LLVMInitializePowerPCTargetMC();
        }
    }

    pub fn initialize_powerpc_asm_printer() {
        unsafe {
            LLVMInitializePowerPCAsmPrinter();
        }
    }

    pub fn initialize_powerpc_asm_parser() {
        unsafe {
            LLVMInitializePowerPCAsmParser();
        }
    }

    pub fn initialize_powerpc_disassembler() {
        unsafe {
            LLVMInitializePowerPCDisassembler();
        }
    }

    pub fn initialize_sparc_target_info() {
        unsafe {
            LLVMInitializeSparcTargetInfo();
        }
    }

    pub fn initialize_sparc_target() {
        unsafe {
            LLVMInitializeSparcTarget();
        }
    }

    pub fn initialize_sparc_target_mc() {
        unsafe {
            LLVMInitializeSparcTargetMC();
        }
    }

    pub fn initialize_sparc_asm_printer() {
        unsafe {
            LLVMInitializeSparcAsmPrinter();
        }
    }

    pub fn initialize_sparc_asm_parser() {
        unsafe {
            LLVMInitializeSparcAsmParser();
        }
    }

    pub fn initialize_sparc_disassembler() {
        unsafe {
            LLVMInitializeSparcDisassembler();
        }
    }

    pub fn initialize_x86_target_info() {
        unsafe {
            LLVMInitializeX86TargetInfo();
        }
    }

    pub fn initialize_x86_target() {
        unsafe {
            LLVMInitializeX86Target();
        }
    }

    pub fn initialize_x86_target_mc() {
        unsafe {
            LLVMInitializeX86TargetMC();
        }
    }

    pub fn initialize_x86_asm_printer() {
        unsafe {
            LLVMInitializeX86AsmPrinter();
        }
    }

    pub fn initialize_x86_asm_parser() {
        unsafe {
            LLVMInitializeX86AsmParser();
        }
    }

    pub fn initialize_x86_disassembler() {
        unsafe {
            LLVMInitializeX86Disassembler();
        }
    }

    pub fn initialize_bpf_target_info() {
        unsafe {
            LLVMInitializeBPFTargetInfo();
        }
    }

    pub fn initialize_bpf_target() {
        unsafe {
            LLVMInitializeBPFTarget();
        }
    }

    pub fn initialize_bpf_target_mc() {
        unsafe {
            LLVMInitializeBPFTargetMC();
        }
    }

    pub fn initialize_bpf_asm_printer() {
        unsafe {
            LLVMInitializeBPFAsmPrinter();
        }
    }

    pub fn initialize_bpf_disassembler() {
        unsafe {
            LLVMInitializeBPFDisassembler();
        }
    }

    pub fn initialize_lanai_target_info() {
        unsafe {
            LLVMInitializeLanaiTargetInfo();
        }
    }

    pub fn initialize_lanai_target() {
        unsafe {
            LLVMInitializeLanaiTarget();
        }
    }

    pub fn initialize_lanai_target_mc() {
        unsafe {
            LLVMInitializeLanaiTargetMC();
        }
    }

    pub fn initialize_lanai_asm_printer() {
        unsafe {
            LLVMInitializeLanaiAsmPrinter();
        }
    }

    pub fn initialize_lanai_asm_parser() {
        unsafe {
            LLVMInitializeLanaiAsmParser();
        }
    }

    pub fn initialize_lanai_disassembler() {
        unsafe {
            LLVMInitializeLanaiDisassembler();
        }
    }

    pub fn initialize_riscv_target_info() {
        unsafe {
            LLVMInitializeRISCVTargetInfo();
        }
    }

    pub fn initialize_riscv_target() {
        unsafe {
            LLVMInitializeRISCVTarget();
        }
    }

    pub fn initialize_riscv_target_mc() {
        unsafe {
            LLVMInitializeRISCVTargetMC();
        }
    }

    pub fn initialize_webassembly_target_info() {
        unsafe {
            LLVMInitializeWebAssemblyTargetInfo();
        }
    }

    pub fn initialize_webassembly_target() {
        unsafe {
            LLVMInitializeWebAssemblyTarget();
        }
    }

    pub fn initialize_webassembly_target_mc() {
        unsafe {
            LLVMInitializeWebAssemblyTargetMC();
        }
    }

    pub fn initialize_webassembly_asm_printer() {
        unsafe {
            LLVMInitializeWebAssemblyAsmPrinter();
        }
    }

    pub fn initialize_webassembly_asm_parser() {
        unsafe {
            LLVMInitializeWebAssemblyAsmParser();
        }
    }

    pub fn initialize_webassembly_disassembler() {
        unsafe {
            LLVMInitializeWebAssemblyDisassembler();
        }
    }
}
