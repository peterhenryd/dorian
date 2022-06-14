use crate::value::Value;

/// Represents a value that is evaluated at compile-time.
#[derive(Debug, Copy, Clone)]
pub struct Const<V: Value>(V);

impl<V: Value> Const<V> {
    /// Creates a [Const<V>] without checking if the given argument is actually a constant.
    pub unsafe fn new_unchecked(value: V) -> Self {
        Const(value)
    }

    /// Borrow the internal value.
    pub fn get_val(&self) -> &V {
        &self.0
    }

    // TODO: pub fn Neg(&self) -> Value;
    // TODO: pub fn NSWNeg(&self) -> Value;
    // TODO: pub fn NUWNeg(&self) -> Value;
    // TODO: pub fn FNeg(&self) -> Value;
    // TODO: pub fn Not(&self) -> Value;
    // TODO: pub fn Add(&self) -> Value;
    // TODO: pub fn NSWAdd(&self) -> Value;
    // TODO: pub fn NUWAdd(&self) -> Value;
    // TODO: pub fn FAdd(&self) -> Value;
    // TODO: pub fn Sub(&self) -> Value;
    // TODO: pub fn NSWSub(&self) -> Value;
    // TODO: pub fn NUWSub(&self) -> Value;
    // TODO: pub fn FSub(&self) -> Value;
    // TODO: pub fn Mul(&self) -> Value;
    // TODO: pub fn NSWMul(&self) -> Value;
    // TODO: pub fn NUWMul(&self) -> Value;
    // TODO: pub fn FMul(&self) -> Value;
    // TODO: pub fn UDiv(&self) -> Value;
    // TODO: pub fn ExactUDiv(&self) -> Value;
    // TODO: pub fn SDiv(&self) -> Value;
    // TODO: pub fn ExactSDiv(&self) -> Value;
    // TODO: pub fn FDiv(&self) -> Value;
    // TODO: pub fn URem(&self) -> Value;
    // TODO: pub fn SRem(&self) -> Value;
    // TODO: pub fn FRem(&self) -> Value;
    // TODO: pub fn And(&self) -> Value;
    // TODO: pub fn Or(&self) -> Value;
    // TODO: pub fn Xor(&self) -> Value;
    // TODO: pub fn ICmp(predicate: IntPredicate, lhs: Value, rhs: Value) -> Value;
    // TODO: pub fn FCmp(predicate: FloatPredicate, lhs: Value, rhs: Value) -> Value;
    // TODO: pub fn Shl(&self) -> Value;
    // TODO: pub fn LShr(&self) -> Value;
    // TODO: pub fn AShr(&self) -> Value;
    // TODO: pub fn GEP2(ty: Type, value: Value, indices: Vec<Value>) -> Value;
    // TODO: pub fn InBoundsGEP2(ty: Type, value: Value, indices: Vec<Value>) -> Value;
    // TODO: pub fn Trunc(&self, to_type: Type) -> Value;
    // TODO: pub fn SExt(&self, to_type: Type) -> Value;
    // TODO: pub fn ZExt(&self, to_type: Type) -> Value;
    // TODO: pub fn FPTrunc(&self, to_type: Type) -> Value;
    // TODO: pub fn FPExt(&self, to_type: Type) -> Value;
    // TODO: pub fn UIToFP(&self, to_type: Type) -> Value;
    // TODO: pub fn SIToFP(&self, to_type: Type) -> Value;
    // TODO: pub fn FPToUI(&self, to_type: Type) -> Value;
    // TODO: pub fn FPToSI(&self, to_type: Type) -> Value;
    // TODO: pub fn PtrToInt(&self, to_type: Type) -> Value;
    // TODO: pub fn IntToPtr(&self, to_type: Type) -> Value;
    // TODO: pub fn BitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn AddrSpaceCast(&self, to_type: Type) -> Value;
    // TODO: pub fn ZExtOrBitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn SExtOrBitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn TruncOrBitCast(&self, to_type: Type) -> Value;
    // TODO: pub fn PointerCast(&self, to_type: Type) -> Value;
    // TODO: pub fn IntCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef, isSigned: LLVMBool) -> LLVMValueRef;
    // TODO: pub fn FPCast(&self, to_type: Type) -> Value;
    // TODO: pub fn Select(ConstantCondition: Value, ConstantIfTrue: Value, ConstantIfFalse: Value,) -> LLVMValueRef;
    // TODO: pub fn ExtractElement(&self, IndexConstant: Value, ) -> LLVMValueRef;
    // TODO: pub fn InsertElement(&self, ElementValueConstant: Value, IndexConstant: LLVMValueRef, ) -> LLVMValueRef;
    // TODO: pub fn ShuffleVector(&self, VectorBConstant: Value, MaskConstant: LLVMValueRef, ) -> LLVMValueRef;
    // TODO: pub fn ExtractValue(AggConstant: Value, IdxList: *mut u32, NumIdx: u32, ) -> LLVMValueRef;
    // TODO: pub fn InsertValue(AggConstant: Value, ElementValueConstant: LLVMValueRef, IdxList: *mut u32, NumIdx: u32, ) -> LLVMValueRef;
}
