use std::mem::{MaybeUninit, transmute};
use std::os::raw::c_char;
use crate::llvm::context::Context;
use crate::llvm::fun::Fun;
use crate::llvm::sys::core::*;
use crate::llvm::sys::LLVMModule;
use crate::llvm::target::triple::TargetTriple;
use crate::llvm::types::Type;
use crate::llvm::{from_c_string, PassManager, to_c_string, VerifierFailureAction};
use std::ptr::NonNull;
use llvm_sys::analysis::LLVMVerifyModule;
use llvm_sys::bit_reader::LLVMGetBitcodeModuleInContext2;
use llvm_sys::bit_writer::{LLVMWriteBitcodeToFD, LLVMWriteBitcodeToFile, LLVMWriteBitcodeToMemoryBuffer};
use llvm_sys::comdat::LLVMGetOrInsertComdat;
use llvm_sys::debuginfo::{LLVMCreateDIBuilder, LLVMCreateDIBuilderDisallowUnresolved, LLVMGetModuleDebugMetadataVersion, LLVMStripModuleDebugInfo};
use crate::llvm::debug::DIBuilder;
use crate::llvm::memory_buffer::MemoryBuffer;
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


    pub fn LLVMGetModuleIdentifier(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMSetModuleIdentifier(
        M: LLVMModuleRef,
        Ident: *const ::libc::c_char,
        Len: ::libc::size_t,
    );
    pub fn LLVMGetSourceFileName(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMSetSourceFileName(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        Len: ::libc::size_t,
    );
    #[deprecated(since = "3.9", note = "Confusingly named. Use LLVMGetDataLayoutStr.")]
    pub fn LLVMGetDataLayout(M: LLVMModuleRef) -> *const ::libc::c_char;
    pub fn LLVMGetDataLayoutStr(M: LLVMModuleRef) -> *const ::libc::c_char;
    pub fn LLVMSetDataLayout(M: LLVMModuleRef, DataLayoutStr: *const ::libc::c_char);
    pub fn LLVMGetTarget(M: LLVMModuleRef) -> *const ::libc::c_char;
    pub fn LLVMSetTarget(M: LLVMModuleRef, Triple: *const ::libc::c_char);
    pub fn LLVMCopyModuleFlagsMetadata(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *mut LLVMModuleFlagEntry;
    pub fn LLVMGetModuleFlag(
        M: LLVMModuleRef,
        Key: *const ::libc::c_char,
        KeyLen: ::libc::size_t,
    ) -> LLVMMetadataRef;
    pub fn LLVMAddModuleFlag(
        M: LLVMModuleRef,
        Behavior: LLVMModuleFlagBehavior,
        Key: *const ::libc::c_char,
        KeyLen: ::libc::size_t,
        Val: LLVMMetadataRef,
    );
    pub fn LLVMDumpModule(M: LLVMModuleRef);
    pub fn LLVMPrintModuleToFile(
        M: LLVMModuleRef,
        Filename: *const ::libc::c_char,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMPrintModuleToString(M: LLVMModuleRef) -> *mut ::libc::c_char;
    pub fn LLVMGetModuleInlineAsm(
        M: LLVMModuleRef,
        Len: *mut ::libc::size_t,
    ) -> *const ::libc::c_char;
    #[deprecated(since = "7.0", note = "Use LLVMSetModuleInlineAsm2 instead")]
    pub fn LLVMSetModuleInlineAsm(M: LLVMModuleRef, Asm: *const ::libc::c_char);
    pub fn LLVMSetModuleInlineAsm2(
        M: LLVMModuleRef,
        Asm: *const ::libc::c_char,
        Len: ::libc::size_t,
    );
    pub fn LLVMAppendModuleInlineAsm(
        M: LLVMModuleRef,
        Asm: *const ::libc::c_char,
        Len: ::libc::size_t,
    );

    pub fn LLVMGetNamedMetadata(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetOrInsertNamedMetadata(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetNamedMetadataName(
        NamedMD: LLVMNamedMDNodeRef,
        NameLen: *const ::libc::size_t,
    ) -> *const ::libc::c_char;
    pub fn LLVMGetNamedMetadataNumOperands(
        M: LLVMModuleRef,
        name: *const ::libc::c_char,
    ) -> ::libc::c_uint;
    pub fn LLVMGetNamedMetadataOperands(
        M: LLVMModuleRef,
        name: *const ::libc::c_char,
        Dest: *mut LLVMValueRef,
    );
    pub fn LLVMAddNamedMetadataOperand(
        M: LLVMModuleRef,
        name: *const ::libc::c_char,
        Val: LLVMValueRef,
    );

    pub fn LLVMGetModuleContext(M: LLVMModuleRef) -> LLVMContextRef;
    #[deprecated(since = "12.0.0", note = "Use LLVMGetTypeByName2 instead")]
    pub fn LLVMGetTypeByName(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMTypeRef;
    pub fn LLVMGetFirstNamedMetadata(M: LLVMModuleRef) -> LLVMNamedMDNodeRef;
    pub fn LLVMGetLastNamedMetadata(M: LLVMModuleRef) -> LLVMNamedMDNodeRef;
    pub fn LLVMAddFunction(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        FunctionTy: LLVMTypeRef,
    ) -> LLVMValueRef;
    pub fn LLVMGetNamedFunction(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetFirstFunction(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastFunction(M: LLVMModuleRef) -> LLVMValueRef;
    // Core->Values->Constants->Global Variables
    pub fn LLVMAddGlobal(
        M: LLVMModuleRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMAddGlobalInAddressSpace(
        M: LLVMModuleRef,
        Ty: LLVMTypeRef,
        Name: *const ::libc::c_char,
        AddressSpace: ::libc::c_uint,
    ) -> LLVMValueRef;
    pub fn LLVMGetNamedGlobal(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetFirstGlobal(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastGlobal(M: LLVMModuleRef) -> LLVMValueRef;


    // Core->Values->Constants->Global Aliases
    pub fn LLVMGetNamedGlobalAlias(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMValueRef;
    pub fn LLVMGetFirstGlobalAlias(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastGlobalAlias(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetNextGlobalAlias(GA: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousGlobalAlias(GA: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMAliasGetAliasee(Alias: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMAliasSetAliasee(Alias: LLVMValueRef, Aliasee: LLVMValueRef);

    pub fn LLVMAddAlias2(
        M: LLVMModuleRef,
        ValueTy: LLVMTypeRef,
        AddrSpace: ::libc::c_uint,
        Aliasee: LLVMValueRef,
        Name: *const ::libc::c_char,
    ) -> LLVMValueRef;
    pub fn LLVMAddGlobalIFunc(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
        Ty: LLVMTypeRef,
        AddrSpace: ::libc::c_uint,
        Resolver: LLVMValueRef,
    ) -> LLVMValueRef;

    /// Obtain a GlobalIFunc value from a Module by its name.
    pub fn LLVMGetNamedGlobalIFunc(
        M: LLVMModuleRef,
        Name: *const ::libc::c_char,
        NameLen: ::libc::size_t,
    ) -> LLVMValueRef;

    /// Obtain an iterator to the first GlobalIFunc in a Module.
    pub fn LLVMGetFirstGlobalIFunc(M: LLVMModuleRef) -> LLVMValueRef;

    /// Obtain an iterator to the last GlobalIFunc in a Module.
    pub fn LLVMGetLastGlobalIFunc(M: LLVMModuleRef) -> LLVMValueRef;

    pub fn LLVMCreateModuleProviderForExistingModule(M: LLVMModuleRef) -> LLVMModuleProviderRef;

    pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> LLVMBool;


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


pub fn LLVMDisposeModuleFlagsMetadata(Entries: *mut LLVMModuleFlagEntry);
pub fn LLVMModuleFlagEntriesGetFlagBehavior(
    Entries: *mut LLVMModuleFlagEntry,
    Index: ::libc::c_uint,
) -> LLVMModuleFlagBehavior;
pub fn LLVMModuleFlagEntriesGetKey(
    Entries: *mut LLVMModuleFlagEntry,
    Index: ::libc::c_uint,
    Len: *mut ::libc::size_t,
) -> *const ::libc::c_char;
pub fn LLVMModuleFlagEntriesGetMetadata(
    Entries: *mut LLVMModuleFlagEntry,
    Index: ::libc::c_uint,
) -> LLVMMetadataRef;

pub fn LLVMGetNextNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;
pub fn LLVMGetPreviousNamedMetadata(NamedMDNode: LLVMNamedMDNodeRef) -> LLVMNamedMDNodeRef;