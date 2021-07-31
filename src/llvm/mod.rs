pub extern crate llvm_sys as sys;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizationLevel {
    None,
    Less,
    Default,
    Aggressive,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressSpace {
    Generic,
    Global,
    Shared,
    Const,
    Local,
}


#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CodeGenFileType {
    AssemblyFile = 0,
    ObjectFile = 1,
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
