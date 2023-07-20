//! Dorian is a Rusty type-safe high-level abstraction for LLVM.
//!
//! # Installing
//!
//! Add Dorian to your cargo project: `cargo add dorian`
//!
//! Dorian uses `llvm-sys` as its sole dependency for LLVM, so we recommend your look at
//! [their page](https://crates.io/crates/llvm-sys/110.0.1) for instructions on how to install LLVM
//! and to make sure that Rust can find it.
//!
//! Check out [EXAMPLES.md](EXAMPLES.md) for a list of simple applications built using Dorian.
//!
//! # Stability
//!
//! Dorian currently offers no guarantee of non-breaking API changes across updates, as changes are
//! constantly being made to improve the utility of the API. Furthermore, the library remains
//! largely untested, so while most of the code *likely* works, we can't guarantee it.
//!
//! Examples are available, and the API is built to be intuitive, so you likely won't run into big
//! issues using Dorian for a hobby project, it is not recommended for a production context.
//!
//! # Transition from LLVM
//!
//! Check out [LLVM_COMPARISON.md](LLVM_COMPARISON.md) for a complete analogous comparison of the
//! LLVM and Dorian API.
//!
//! # Contributing
//!
//! Check out [CONTRIBUTING.md](CONTRIBUTING.md)
//!
//! # License
//!
//! Dorian is licensed under the MIT license, found [here](LICENSE).

pub mod dorian;
pub mod fun;
pub mod llvm;
pub mod module;
pub mod types;
pub mod value;

pub mod prelude {
    pub use crate::dorian::Dorian;
    pub use crate::fun::{
        block::Block,
        Fun
    };
    pub use crate::llvm::{
        IntPredicate,
        FloatPredicate,
        AtomicOrdering,
        AtomicRMWBinOp,
        OptimizationLevel,
        AddressSpace,
        RelocMode,
        CodeModel,
        CodeGenFileType,
        Linkage,
        ThreadLocalMode,
        ModuleFlagBehavior,
        InlineAsmDialect
    };
    pub use crate::module::Module;
    pub use crate::types::{
        Type,
        CreateType,
        array::{ArrayData, ArrayType},
        float::{FloatData, FloatType},
        fun::{FunData, FunType},
        int::{IntData, IntType},
        ptr::{PtrData, PtrType},
        structure::{NamedStructData, StructData, StructType},
        vector::{VectorData, VectorType},
        void::{VoidData, VoidType},
    };
    pub use crate::value::{
        Value,
        cmp::CmpValue,
        constant::Const,
        data::BuildValue,
        float::{BinOp as FloatBinOp, UnaOp as FloatUnaOp, FloatValue, Float},
        fun::FunValue,
        int::{BinOp as IntBinOp, Int, IntValue, UnaOp as IntUnaOp},
        ptr::{BinOp as PtrBinOp, Deref, Ptr, PtrValue, UnaOp as PtrUnaOp},
        vector::VectorValue
    };
}