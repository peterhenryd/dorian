use inkwell::AddressSpace;
use inkwell::types::AnyTypeEnum;
use inkwell::values::{AnyValue as InkwellValue, PointerValue};
use crate::fun::block::Block;
use crate::types::ptr::PtrType;
use crate::types::Type;
use crate::value::data::BuildValue;
use crate::value::{NonAnyValue, Value};

/// Represents a pointer value.
#[derive(Debug, Copy, Clone)]
pub struct PtrValue<'a, V: Value<'a> + Copy + Clone>(PointerValue<'a>, PtrType<'a, V::Type>);

impl<'a, V: Value<'a> + Copy + Clone> Value<'a> for PtrValue<'a, V> where V::Type: Copy + Clone {
    type Type = PtrType<'a, V::Type>;
    type InkwellValue = PointerValue<'a>;

    unsafe fn new_unchecked(value: PointerValue<'a>,  base_type: PtrType<'a, V::Type>) -> PtrValue<'a, V> {
        PtrValue(value, base_type)
    }

    fn get_inkwell_value(&self) -> Self::InkwellValue {
        self.0
    }

    fn get_type(&self) -> &Self::Type {
        &self.1
    }
}

impl<'a, V: Value<'a> + Copy + Clone> NonAnyValue<'a> for PtrValue<'a, V> where V::Type: Copy + Clone {}

/// Represents a binary operation that an pointer value may undergo.
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

/// Represents a unary operation that an pointer value may undergo.
pub enum UnaOp {
    Not,
    Neg,
    NswNeg,
    NuwNeg,
}

/// Represents ways for instantiating a pointer value.
pub enum Ptr<'a, V: Value<'a>> {
    Ref(&'a V, AddressSpace),
    Alloc(&'a V, AddressSpace),
}

impl<'a, V: Value<'a> + Copy + Clone> PtrValue<'a, V> {
    pub fn deref(&self) -> Deref<'a, V> {
        Deref(*self)
    }
}

impl<'a, V: Value<'a> + Copy + Clone> BuildValue for Ptr<'a, V> where V::Type: Copy {
    type Value = PtrValue<'a, V>;

    fn build<R: Type<'a>>(&self, block: &Block<'a, R>) -> Self::Value<'a> {
        match self {
            Ptr::Ref(val, address_space) => {
                let t = val.get_type().get_inkwell_type().to_ptr_type(*address_space);
                unsafe {
                    let ptr = block.get_builder().build_alloca(t, ""); //todo: names
                    block.get_builder().build_store(val.get_inkwell_value(), ptr);
                    PtrValue::new_unchecked(
                        ptr,
                        PtrType::from_inkwell_type_unchecked(
                            val.get_type().get_inkwell_type().to_ptr_type(*address_space),
                        ),
                    )
                }
            }
            Ptr::Alloc(val, address_space) => {
                let t = val.get_type().get_inkwell_type().to_ptr_type(*address_space);
                unsafe {
                    let ptr = block.get_builder().build_malloc(t, "");  //todo: names
                    block.get_builder().build_store(val.get_inkwell_value(), ptr);
                    PtrValue::new_unchecked(
                        ptr,
                        PtrType::from_inkwell_type_unchecked(
                            val.get_type().get_inkwell_type().to_ptr_type(*address_space),
                        ),
                    )
                }
            }
        }
    }
}


/// Represents the de-referencing of a pointer value.
#[derive(Debug, Copy, Clone)]
pub struct Deref<'a, V: Value<'a> + Copy + Clone>(PtrValue<'a, V>);

impl<'a, V: Value<'a> + Copy + Clone> BuildValue for Deref<'a, V> where V::Type: Copy {
    type Value<'b> = V;

    fn build<R: Type<'a>>(&self, block: &Block<'a, R>) -> Self::Value<'a> {
        let val = self.0.get_inkwell_value();

        unsafe {
            V::new_unchecked(
                block.get_builder().build_load(val, "").as_any_value_enum(),  //todo: names
                Type::from_inkwell_type_unchecked(
                    val.get_type().get_pointing_type()
                )
            )
        }
    }
}
