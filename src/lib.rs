pub extern crate dorian_ast as ast;
#[cfg(feature = "cranelift")]
pub extern crate dorian_cranelift as cranelift;
#[cfg(feature = "llvm")]
pub extern crate dorian_llvm as llvm;

pub mod prelude {
    pub use crate::ast::block::Block;
    pub use crate::ast::block::builder::*;
    pub use crate::ast::block::stmt::*;
    pub use crate::ast::ty::{Type, ConcreteType, DataType, ScalarType, NumType, IntType, IntWidth, FloatType, BoolType,
                             PtrType, VectorType, VoidType};  
    pub use crate::ast::ty::util as ty;
    pub use crate::ast::val::{Value, ContextValue, Arg, Var, Expr, Bin, BinOp, Una, UnaOp, Lit, Num, Int, SignedInt, 
                              UnsignedInt, Float, Call};
    pub use crate::ast::val::util as val;
    pub use crate::ast::backend::*;
    pub use crate::ast::function::*;
    pub use crate::ast::global::*;
    pub use crate::ast::module::*;
    pub use crate::ast::structure::*;
    
    #[cfg(feature = "llvm")]
    pub use crate::llvm::Llvm;
    
    #[cfg(feature = "cranelift")]
    pub use crate::cranelift::Cranelift;
}