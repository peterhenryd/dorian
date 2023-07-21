pub mod block;

use inkwell::values::FunctionValue;
use crate::dorian::Dorian;
use crate::fun::block::Block;
use crate::types::{RawType, Type};
use crate::value::any::AnyValue;
use crate::value::Value;

/// This structure represents a function in the context of a module.
pub struct Fun<'a, R: Type<'a>> {
    dorian: &'a Dorian,
    inkwell_function_value: FunctionValue<'a>,
    return_type: R,
}

impl<'a, R: Type<'a>> Fun<'a, R> {
    /// Creates a function from an [LlvmFun].
    pub fn from_inkwell(dorian: &'a Dorian, inkwell_function_value: FunctionValue<'a>, return_type: R) -> Fun<'a, R> {
        Fun {
            dorian, inkwell_function_value, return_type
        }
    }

    pub fn get_function_value(&self) -> &FunctionValue<'a> {
        &self.1
    }

    pub fn get_return_type(&self) -> &R {
        &self.return_type
    }

    /// Add a named block to the function.
    pub fn add_block(&mut self, name: &str) -> Block<'a, R> {
        Block::new(
            self.dorian,
            self.dorian.get_context().create_builder(),
            self.1.append_basic_block(name),
        )
    }

    /// Returns the values inputted when the function is called.
    pub fn fetch_params(&self) -> Vec<AnyValue> {
        self.1
            .get_params()
            .iter()
            .map(|&v| unsafe {
                AnyValue::new_unchecked(v, RawType::from_inkwell_type_unchecked(v.get_type()))
            })
            .collect()
    }
}
