pub extern crate llvm_sys as sys;

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::ptr::NonNull;
use llvm_sys::core::{LLVMCreatePassManager, LLVMDisposePassManager, LLVMFinalizeFunctionPassManager, LLVMInitializeFunctionPassManager, LLVMRunFunctionPassManager, LLVMRunPassManager, LLVMShutdown};
use llvm_sys::error::{LLVMConsumeError, LLVMCreateStringError, LLVMGetErrorMessage, LLVMGetErrorTypeId, LLVMOpaqueError};
use llvm_sys::{LLVMPassManager, LLVMPassRegistry};
use llvm_sys::initialization::{LLVMInitializeAggressiveInstCombiner, LLVMInitializeAnalysis, LLVMInitializeCodeGen, LLVMInitializeCore, LLVMInitializeInstCombine, LLVMInitializeInstrumentation, LLVMInitializeIPA, LLVMInitializeIPO, LLVMInitializeObjCARCOpts, LLVMInitializeScalarOpts, LLVMInitializeTarget, LLVMInitializeTransformUtils, LLVMInitializeVectorization};
use crate::llvm::fun::Fun;
use crate::llvm::module::Module;

pub mod basic_block;
pub mod builder;
pub mod context;
pub mod execution_engine;
pub mod fun;
pub mod module;
pub mod opcode;
pub mod target;
pub mod types;
pub mod value;
pub mod memory_buffer;
pub mod debug;
pub mod disassembler;
pub mod lto;
pub mod object;
pub mod orc2;
pub mod remarks;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntPredicate {
    Eq = 32,
    Ne = 33,
    Ugt = 34,
    Uge = 35,
    Ult = 36,
    Ule = 37,
    Sgt = 38,
    Sge = 39,
    Slt = 40,
    Sle = 41,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FloatPredicate {
    False = 0,
    Oeq = 1,
    Ogt = 2,
    Oge = 3,
    Olt = 4,
    Ole = 5,
    One = 6,
    Ord = 7,
    Uno = 8,
    Ueq = 9,
    Ugt = 10,
    Uge = 11,
    Ult = 12,
    Ule = 13,
    Une = 14,
    True = 15,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtomicOrdering {
    NotAtomic = 0,
    Unordered = 1,
    Monotonic = 2,
    Acquire = 4,
    Release = 5,
    AcquireRelease = 6,
    SequentiallyConsistent = 7,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtomicRMWBinOp {
    Xchg = 0,
    Add = 1,
    Sub = 2,
    And = 3,
    Nand = 4,
    Or = 5,
    Xor = 6,
    Max = 7,
    Min = 8,
    UMax = 9,
    UMin = 10,
    FAdd = 11,
    FSub = 12,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    None,
    Less,
    Default,
    Aggressive,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressSpace {
    Generic,
    Global,
    Shared,
    Const,
    Local,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelocMode {
    Default = 0,
    Static = 1,
    PIC = 2,
    DynamicNoPic = 3,
    Ropi = 4,
    Rwpi = 5,
    RopiRwpi = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodeModel {
    Default = 0,
    JitDefault = 1,
    Tiny = 2,
    Small = 3,
    Kernel = 4,
    Medium = 5,
    Large = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodeGenFileType {
    Assembly = 0,
    Object = 1,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LLVMLinkage {
    External = 0,
    AvailableExternally = 1,
    LinkOnceAny = 2,
    LinkOnceODR = 3,
    LinkOnceODRAutoHide = 4,
    WeakAny = 5,
    WeakODR = 6,
    Appending = 7,
    Internal = 8,
    Private = 9,
    DLLImport = 10,
    DLLExport = 11,
    ExternalWeak = 12,
    Ghost = 13,
    Common = 14,
    LinkerPrivate = 15,
    LinkerPrivateWeak = 16,
}

#[inline(always)]
fn to_c_string(str: Option<&str>) -> CString {
    CString::new(str.or_else(|| Some("")).unwrap()).expect("error creating CString")
}

#[inline(always)]
fn from_c_string<'a>(ptr: *const c_char) -> &'a str {
    unsafe {
        CStr::from_ptr(ptr)
            .to_str()
            .expect("received CString is not valid UTF-8")
    }
}

// TODO: kinda unsure about all of this verifier stuff and if it's really necessary
#[repr(C)]
pub enum VerifierFailureAction {
    /// Print to stderr and abort the process.
    AbortProcessAction = 0,
    /// Print to stderr and return 1.
    PrintMessageAction = 1,
    /// Return 1 and print nothing.
    ReturnStatusAction = 2,
}

// TODO: idk if i like this
pub fn shutdown() {
    unsafe {
        LLVMShutdown();
    }
}

// TODO: do we need these?
// pub fn LLVMCreateMessage(Message: *const ::libc::c_char) -> *mut ::libc::c_char;
//  pub fn LLVMDisposeMessage(Message: *mut ::libc::c_char);

#[derive(Debug)]
pub struct Error(NonNull<LLVMOpaqueError>);

impl Error {
    #[inline]
    pub fn from_raw(raw: NonNull<LLVMOpaqueError>) -> Error {
        Error(raw)
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<LLVMOpaqueError> {
        self.1
    }

    pub fn create(message: &str) -> Error {
        Error(unsafe {
            NonNull::new_unchecked(
                LLVMCreateStringError(
                    to_c_string(Some(message)).as_ptr()
                )
            )
        })
    }

    // TODO: make this nicer
    pub fn get_type_id(&self) -> *const c_void {
        unsafe {
            LLVMGetErrorTypeId(self.0.as_ptr())
        }
    }

    pub fn get_message(&self) -> &str {
        from_c_string(unsafe {
            LLVMGetErrorMessage(
                self.0.as_ptr()
            )
        })
    }

    // TODO: pub fn DisposeErrorMessage(ErrMsg: *mut ::libc::c_char);
    // TODO: pub fn GetStringErrorTypeId() -> LLVMErrorTypeId;
}

// TODO: unsure if this is correct
impl Drop for Error {
    fn drop(&mut self) {
        unsafe {
            LLVMConsumeError(self.0.as_ptr());
        }
    }
}

pub type LLVMFatalErrorHandler = Option<extern "C" fn(Reason: *const ::libc::c_char)>;

pub fn LLVMInstallFatalErrorHandler(Handler: LLVMFatalErrorHandler);
pub fn LLVMResetFatalErrorHandler();
pub fn LLVMEnablePrettyStackTrace();

#[derive(Debug)]
pub struct PassRegistry(NonNull<LLVMPassRegistry>);

impl PassRegistry {
    pub fn initialize_core(&self) {
        unsafe { LLVMInitializeCore(self.0.as_ptr()); }
    }

    pub fn initialize_transform_utils(&self) {
        unsafe { LLVMInitializeTransformUtils(self.0.as_ptr()); }
    }

    pub fn initialize_scalar_opts(&self) {
        unsafe { LLVMInitializeScalarOpts(self.0.as_ptr()); }
    }

    pub fn initialize_obj_carc_opts(&self) {
        unsafe { LLVMInitializeObjCARCOpts(self.0.as_ptr()); }
    }

    pub fn initialize_vectorization(&self) {
        unsafe { LLVMInitializeVectorization(self.0.as_ptr()); }
    }

    pub fn initialize_inst_combine(&self) {
        unsafe { LLVMInitializeInstCombine(self.0.as_ptr()); }
    }

    pub fn initialize_aggressive_inst_combiner(&self) {
        unsafe { LLVMInitializeAggressiveInstCombiner(self.0.as_ptr()); }
    }

    pub fn initialize_ipo(&self) {
        unsafe { LLVMInitializeIPO(self.0.as_ptr()); }
    }

    pub fn initialize_instrumentation(&self) {
        unsafe { LLVMInitializeInstrumentation(self.0.as_ptr()); }
    }

    pub fn initialize_analysis(&self) {
        unsafe { LLVMInitializeAnalysis(self.0.as_ptr()); }
    }

    pub fn initialize_ipa(&self) {
        unsafe { LLVMInitializeIPA(self.0.as_ptr()); }
    }

    pub fn initialize_code_gen(&self) {
        unsafe { LLVMInitializeCodeGen(self.0.as_ptr()); }
    }

    pub fn initialize_target(&self) {
        unsafe { LLVMInitializeTarget(self.0.as_ptr()); }
    }
}

pub fn load_library_permanently(filename: &str) -> bool;
pub fn parse_command_line_options(
    argv: Vec<&str>,
    overview: &str
);

pub fn search_for_address_of_symbol(symbolName: *const ::libc::c_char) -> *mut ::libc::c_void;
pub fn add_symbol(name: &str, value: *mut c_void);

#[derive(Debug)]
pub struct PassManager(NonNull<LLVMPassManager>);

impl PassManager {
    pub fn from_raw(raw: NonNull<LLVMPassManager>) -> PassManager {
        PassManager(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMPassManager> {
        self.0
    }

    pub fn new() -> PassManager {
        PassManager(unsafe {
            NonNull::new_unchecked(
                LLVMCreatePassManager()
            )
        })
    }

    pub fn run(&self, module: Module) -> bool {
        unsafe {
            LLVMRunPassManager(self.0.as_ptr(), module.as_raw().as_ptr()) != 0
        }
    }

    pub fn initialize_function(&self) -> bool {
        unsafe {
            LLVMInitializeFunctionPassManager(self.0.as_ptr()) != 0
        }
    }

    pub fn finalize_function(&self) -> bool {
        unsafe {
            LLVMFinalizeFunctionPassManager(self.0.as_ptr()) != 0
        }
    }

    pub fn run_function(&self, fun: Fun) -> bool {
        unsafe {
            LLVMRunFunctionPassManager(self.0.as_ptr(), fun.as_raw().as_ptr()) != 0
        }
    }
}

impl Drop for PassManager {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassManager(self.0.as_ptr())
        }
    }
}