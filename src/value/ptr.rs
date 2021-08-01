use crate::fun::block::Block;
use crate::llvm::AddressSpace;
use crate::types::ptr::PtrType;
use crate::types::Type;
use crate::value::data::BuildValue;
use crate::value::{LlvmValue, Value};

#[derive(Debug, Copy, Clone)]
pub struct PtrValue<V: Value + Copy + Clone>(LlvmValue, PtrType<V::Type>, );

impl<V: Value + Copy + Clone> Value for PtrValue<V> where V::Type: Copy + Clone {
    type Type = PtrType<V::Type>;

    unsafe fn new_unchecked(value: LlvmValue, base_type: PtrType<V::Type>) -> PtrValue<V> {
        PtrValue(value, base_type)
    }

    fn get_llvm_value(&self) -> LlvmValue {
        self.0
    }

    fn get_type(&self) -> &PtrType<V::Type> {
        &self.1
    }
}

pub enum BinOp {
    Add,
    NuwAdd,
    NswAdd,
    Sub,
    NuwSub,
    NswSub,
    Mul,
    NuwMul,
    NswMul,
    UDiv,
    SDiv,
    ExactUDiv,
    ExactSDiv,
    URem,
    SRem,
    Shl,
    LShr,
    AShr,
    And,
    Or,
    Xor,
}

pub enum UnaOp {
    Not,
    Neg,
    NswNeg,
    NuwNeg,
}

pub enum Ptr<'a, V: Value> {
    Ref(&'a V, AddressSpace),
    Alloc(&'a V, AddressSpace),
}

impl<V: Value + Copy + Clone> PtrValue<V> {
    pub fn deref(&self) -> Deref<V> {
        Deref(self)
    }
}

impl<'a, V: Value + Copy + Clone> BuildValue<'a> for Ptr<'a, V> where V::Type: Copy {
    type Value = PtrValue<V>;

    fn build<R: Type>(&self, block: &Block<R>) -> Self::Value {
        match self {
            Ptr::Ref(val, address_space) => {
                let t = val.get_type().get_llvm_type().to_ptr_type(*address_space);
                unsafe {
                    let ptr = block.get_builder().build_alloca(t, None);
                    block.get_builder().build_store(val.get_llvm_value(), ptr);
                    PtrValue::new_unchecked(
                        ptr,
                        PtrType::from_llvm_type_unchecked(
                            val.get_type().get_llvm_type().to_ptr_type(*address_space),
                        ),
                    )
                }
            }
            Ptr::Alloc(val, address_space) => {
                let t = val.get_type().get_llvm_type().to_ptr_type(*address_space);
                unsafe {
                    let ptr = block.get_builder().build_malloc(t, None);
                    block.get_builder().build_store(val.get_llvm_value(), ptr);
                    PtrValue::new_unchecked(
                        ptr,
                        PtrType::from_llvm_type_unchecked(
                            val.get_type().get_llvm_type().to_ptr_type(*address_space),
                        ),
                    )
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Deref<'a, V: Value + Copy + Clone>(&'a PtrValue<V>);

impl<'a, V: Value + Copy + Clone> BuildValue<'a> for Deref<'a, V> where V::Type: Copy {
    type Value = V;

    fn build<R: Type>(&self, block: &Block<R>) -> Self::Value {
        let val = self.0.get_llvm_value();

        unsafe {
            V::new_unchecked(
                block.get_builder().build_load(val, None),
                V::Type::from_llvm_type_unchecked(
                    val.get_type().get_pointing_type()
                )
            )
        }
    }
}
