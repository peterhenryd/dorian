use std::mem::{MaybeUninit, transmute};
use std::os::raw::c_char;
use crate::llvm::context::Context;
use crate::llvm::fun::Fun;
use crate::llvm::sys::core::*;
use crate::llvm::sys::LLVMModule;
use crate::llvm::target::triple::TargetTriple;
use crate::llvm::types::Type;
use crate::llvm::{Error, from_c_string, ModuleFlagBehavior, PassManager, to_c_string, VerifierFailureAction};
use std::ptr::NonNull;
use std::slice;
use llvm_sys::analysis::LLVMVerifyModule;
use llvm_sys::bit_reader::LLVMGetBitcodeModuleInContext2;
use llvm_sys::bit_writer::{LLVMWriteBitcodeToFD, LLVMWriteBitcodeToFile, LLVMWriteBitcodeToMemoryBuffer};
use llvm_sys::comdat::LLVMGetOrInsertComdat;
use llvm_sys::debuginfo::{LLVMCreateDIBuilder, LLVMCreateDIBuilderDisallowUnresolved, LLVMGetModuleDebugMetadataVersion, LLVMStripModuleDebugInfo};
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::LLVMModuleFlagEntry;
use llvm_sys::target::{LLVMGetModuleDataLayout, LLVMSetModuleDataLayout};
use crate::llvm::debug::{DIBuilder, Metadata};
use crate::llvm::memory_buffer::MemoryBuffer;
use crate::llvm::target::TargetData;
use crate::llvm::value::Comdat;

pub struct Module<'a>(&'a Context, NonNull<LLVMModule>);

impl<'a> Module<'a> {
    #[inline]
    pub fn from_raw(context: &'a Context, module: NonNull<LLVMModule>) -> Module<'a> {
        Module(context, module)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMModule> {
        self.1
    }

    pub fn from_bitcode(context: &'a Context, memory_buffer: MemoryBuffer) -> Module<'a> {
        unsafe {
            let mut module = MaybeUninit::uninit();

            LLVMGetBitcodeModuleInContext2(
                context.as_raw().as_ptr(),
                memory_buffer.as_raw().as_ptr(),
                module.as_mut_ptr()
            );

            Module(context, NonNull::new_unchecked(module.assume_init()))
        }
    }

    pub fn set_source_file_name(&mut self, name: &str) {
        unsafe {
            LLVMSetSourceFileName(
                self.1.as_ptr(),
                to_c_string(Some(name)).as_ptr(),
                name.len() + 1,
            );
        }
    }

    pub fn get_data_layout(&self) -> &str {
        unsafe { from_c_string(LLVMGetDataLayoutStr(self.1.as_ptr())) }
    }

    pub fn set_data_layout(&mut self, data_layout: &str) {
        unsafe { LLVMSetDataLayout(self.1.as_ptr(), to_c_string(Some(data_layout)).as_ptr()) }
    }

    pub fn get_target_triple(&self) -> TargetTriple {
        TargetTriple::from_str(unsafe { from_c_string(LLVMGetTarget(self.1.as_ptr())) })
    }

    pub fn set_target(&mut self, target_triple: TargetTriple) {
        unsafe {
            LLVMSetTarget(
                self.1.as_ptr(),
                to_c_string(Some(target_triple.as_str())).as_ptr(),
            )
        }
    }

    pub fn get_context(&self) -> &Context {
        self.0
    }

    pub unsafe fn add_fun(&mut self, name: &str, fun_type: Type) -> Fun<'a> {
        Fun::<'a>::from_raw(
            self.0,
            NonNull::new_unchecked(LLVMAddFunction(
                self.1.as_ptr(),
                to_c_string(Some(name)).as_ptr(),
                fun_type.as_raw().as_ptr(),
            )),
            fun_type
        )
    }

    pub fn verify(&self, action: VerifierFailureAction) -> bool {
        unsafe {
            LLVMVerifyModule(
                self.as_raw().as_ptr(),
                transmute(action),
                transmute::<*const *mut c_char, *mut *mut c_char>(Vec::new().as_ptr())
            ) != 0
        }
    }

    pub fn write_bitcode_to_file_descriptor(&self, file_descriptor: i32, should_close: i32, unbuffered: i32) -> bool {
        unsafe { LLVMWriteBitcodeToFD(self.1.as_ptr(), file_descriptor, should_close, unbuffered) == 0 }
    }

    pub fn write_bitcode_to_file(&self, path: &str) -> bool {
        unsafe { LLVMWriteBitcodeToFile(self.1.as_ptr(), to_c_string(Some(path)).as_ptr()) == 0 }
    }

    pub fn write_bitcode_to_memory_buffer(&self) -> MemoryBuffer {
        MemoryBuffer::from_raw( unsafe {
            NonNull::new_unchecked(
                LLVMWriteBitcodeToMemoryBuffer(self.1.as_ptr())
            )
        })
    }

    pub fn get_or_insert_comdat(&self, name: &str) -> Comdat {
        Comdat::from_raw(unsafe {
            NonNull::new_unchecked(
             LLVMGetOrInsertComdat(self.1.as_ptr(), to_c_string(Some(name)).as_ptr())
            )
        })
    }

    pub fn get_module_debug_metadata_version(&self) -> usize {
        unsafe {
            LLVMGetModuleDebugMetadataVersion(self.1.as_ptr()) as usize
        }
    }

    pub fn strip_module_debug_info(&self) -> bool {
        unsafe {
            LLVMStripModuleDebugInfo(self.1.as_ptr()) != 0
        }
    }

    pub fn create_dibuilder_disallow_unresolved(&self) -> DIBuilder {
        DIBuilder::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateDIBuilderDisallowUnresolved(self.1.as_ptr())
            )
        })
    }

    pub fn create_dibuilder(&self) -> DIBuilder {
        DIBuilder::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateDIBuilder(self.1.as_ptr())
            )
        })
    }

    pub fn create_function_pass_manager(&self) -> PassManager {
        PassManager::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateFunctionPassManagerForModule(self.1.as_ptr())
            )
        })
    }

    pub fn get_module_identifier(&self) -> String {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let chars = LLVMGetModuleIdentifier(self.1.as_ptr(), len.as_mut_ptr());

            String::from_utf8(
                transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init())).to_vec()
            ).unwrap()
        }
    }

    pub fn set_module_identifier(&self, str: &str) {
        unsafe { LLVMSetModuleIdentifier(self.1.as_ptr(), to_c_string(Some(str)).as_ptr(), str.len()) }
    }

    pub fn get_source_file_name(&self) -> String {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let chars = LLVMGetSourceFileName(self.1.as_ptr(), len.as_mut_ptr());

            String::from_utf8(
                transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init())).to_vec()
            ).unwrap()
        }
    }

    pub fn get_data_layout_str(&self) -> &str {
        unsafe { from_c_string(LLVMGetDataLayoutStr(self.1.as_ptr())) }
    }

    pub fn get_target(&self) -> TargetTriple {
        TargetTriple::from_str(unsafe { from_c_string(LLVMGetTarget(self.1.as_ptr())) })
    }

    pub fn copy_module_flags_metadata(&self) -> ModuleFlagEntries {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let flags = LLVMCopyModuleFlagsMetadata(self.1.as_ptr(), len.as_mut_ptr());

            ModuleFlagEntries(flags, len.assume_init())
        }
    }

    pub fn get_module_flag(&self, key: &str) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetModuleFlag(self.1.as_ptr(), to_c_string(Some(key)).as_ptr(), key.len())
            )
        })
    }

    pub fn add_module_flag(&self, behavior: ModuleFlagBehavior, key: &str, value: Metadata) {
        unsafe {
            LLVMAddModuleFlag(
                self.1.as_ptr(),
                transmute(behavior),
                to_c_string(Some(key)).as_ptr(),
                key.len(),
                value.as_raw().as_ptr()
            )
        }
    }

    pub fn dump_module(&self) {
        unsafe { LLVMDumpModule(self.1.as_ptr()); }
    }

    pub fn print_module_to_file(&self, filename: &str) -> Result<(), Error> {
        unsafe {
            let mut message = MaybeUninit::uninit();
            let result = LLVMPrintModuleToFile(self.1.as_ptr(), to_c_string(Some(filename)).as_ptr(), message.as_mut_ptr()) ;

            // Verify if condition is valid
            if result == 0 {
                Ok(())
            } else {
                Err(Error::create(from_c_string(message.assume_init())))
            }
        }
    }

    pub fn print_module_to_string(&self) -> &str {
        unsafe { from_c_string(LLVMPrintModuleToString(self.1.as_ptr())) }
    }

    pub fn get_module_inline_asm(&self) -> String {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let chars = LLVMGetModuleInlineAsm(self.1.as_ptr(), len.as_mut_ptr());

            String::from_utf8(
                transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init())).to_vec()
            ).unwrap()
        }
    }

    pub fn set_module_inline_asm(&self, asm: &str) {
        unsafe { LLVMSetModuleInlineAsm2(self.1.as_ptr(), to_c_string(Some(asm)).as_ptr(), asm.len()); }
    }

    pub fn append_module_inline_asm(&self, asm: &str) {
        unsafe { LLVMAppendModuleInlineAsm(self.1.as_ptr(), to_c_string(Some(asm)).as_ptr(), asm.len()); }
    }

    // TODO: pub fn GetNamedMetadata(&self, name: &str) -> LLVMNamedMDNodeRef;
    // TODO: pub fn GetOrInsertNamedMetadata(&self, Name: *const ::libc::c_char, NameLen: ::libc::size_t, ) -> LLVMNamedMDNodeRef;
    // TODO: pub fn GetNamedMetadataName(&self, NameLen: *const ::libc::size_t, ) -> *const ::libc::c_char;
    // TODO: pub fn GetNamedMetadataNumOperands(&self, name: *const ::libc::c_char, ) -> ::libc::c_uint;
    // TODO: pub fn GetNamedMetadataOperands(&self, name: *const ::libc::c_char, Dest: *mut LLVMValueRef, );
    // TODO: pub fn AddNamedMetadataOperand(&self, name: *const ::libc::c_char, Val: LLVMValueRef, );

    // TODO: pub fn GetModuleContext(&self) -> LLVMContextRef;
    // TODO: pub fn GetFirstNamedMetadata(&self) -> LLVMNamedMDNodeRef;
    // TODO: pub fn GetLastNamedMetadata(&self) -> LLVMNamedMDNodeRef;
    // TODO: pub fn AddFunction(&self, Name: *const ::libc::c_char, FunctionTy: LLVMTypeRef, ) -> LLVMValueRef;
    // TODO: pub fn GetNamedFunction(&self, Name: *const ::libc::c_char) -> LLVMValueRef;
    // TODO: pub fn GetFirstFunction(&self) -> LLVMValueRef;
    // TODO: pub fn GetLastFunction(&self) -> LLVMValueRef;
    // TODO: // Core->Values->Constants->Global Variables
    // TODO: pub fn AddGlobal(&self, Ty: LLVMTypeRef, Name: *const ::libc::c_char, ) -> LLVMValueRef;
    // TODO: pub fn AddGlobalInAddressSpace(&self, Ty: LLVMTypeRef, Name: *const ::libc::c_char, AddressSpace: ::libc::c_uint, ) -> LLVMValueRef;
    // TODO: pub fn GetNamedGlobal(&self, Name: *const ::libc::c_char) -> LLVMValueRef;
    // TODO: pub fn GetFirstGlobal(&self) -> LLVMValueRef;
    // TODO: pub fn GetLastGlobal(&self) -> LLVMValueRef;

    // TODO: // Core->Values->Constants->Global Aliases
    // TODO: pub fn GetNamedGlobalAlias(&self, Name: *const ::libc::c_char, NameLen: ::libc::size_t, ) -> LLVMValueRef;
    // TODO: pub fn GetFirstGlobalAlias(&self) -> Value;
    // TODO: pub fn GetLastGlobalAlias(&self) -> LLVMValueRef;

    // TODO: pub fn LLVMAddAlias2(&self, ValueTy: LLVMTypeRef, AddrSpace: ::libc::c_uint, Aliasee: LLVMValueRef, Name: *const ::libc::c_char, ) -> LLVMValueRef;
    // TODO: pub fn LLVMAddGlobalIFunc(&self, Name: *const ::libc::c_char, NameLen: ::libc::size_t, Ty: LLVMTypeRef, AddrSpace: ::libc::c_uint, Resolver: LLVMValueRef, ) -> LLVMValueRef;
    // TODO: pub fn LLVMGetNamedGlobalIFunc(&self, Name: *const ::libc::c_char, NameLen: ::libc::size_t, ) -> LLVMValueRef;
    // TODO: pub fn LLVMGetFirstGlobalIFunc(&self) -> LLVMValueRef;
    // TODO: pub fn LLVMGetLastGlobalIFunc(&self) -> LLVMValueRef;
    // TODO: pub fn LLVMCreateModuleProviderForExistingModule(&self) -> LLVMModuleProviderRef;

    pub fn link(dest: Module, src: Module) -> bool {
        unsafe {
            LLVMLinkModules2(dest.1.as_ptr(), src.1.as_ptr()) != 0
        }
    }

    pub fn get_module_data_layout(&self) -> TargetData {
        TargetData::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetModuleDataLayout(self.1.as_ptr())
            )
        })
    }
    pub fn set_module_data_layout(&self, target_data: TargetData) {
        unsafe { LLVMSetModuleDataLayout(self.1.as_ptr(), target_data.as_raw().as_ptr()); }
    }
}

impl ToString for Module<'_> {
    fn to_string(&self) -> String {
        unsafe { from_c_string(LLVMPrintModuleToString(self.1.as_ptr())).to_string() }
    }
}

impl Drop for Module<'_> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.1.as_ptr());
        }
    }
}

impl Clone for Module<'_> {
    fn clone(&self) -> Self {
        unsafe {
            Module::from_raw(
                self.0,
                NonNull::new_unchecked(LLVMCloneModule(self.1.as_ptr())),
            )
        }
    }
}

pub struct ModuleFlagEntries(*mut LLVMModuleFlagEntry, usize);

impl ModuleFlagEntries {
    pub fn get_flag_behavior(&self, index: u32) -> ModuleFlagBehavior {
        unsafe {
            transmute(LLVMModuleFlagEntriesGetFlagBehavior(
                self.0, index,
            ))
        }
    }

    pub fn get_key(&self, index: u32) -> String {
        unsafe {
            let mut len = MaybeUninit::uninit();
            let chars = LLVMModuleFlagEntriesGetKey(self.0, index, len.as_mut_ptr());

            String::from_utf8(
                transmute::<&[i8], &[u8]>(slice::from_raw_parts(chars, len.assume_init())).to_vec()
            ).unwrap()
        }
    }

    pub fn get_metadata(&self, index: u32) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMModuleFlagEntriesGetMetadata(self.0, index)
            )
        })
    }
}

impl Drop for ModuleFlagEntries {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModuleFlagsMetadata(self.0); }
    }
}

// TODO: pub fn LLVMGetNextNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;
// TODO: pub fn LLVMGetPreviousNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;