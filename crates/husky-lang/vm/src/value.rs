mod any;
mod owned;
mod primitive;
mod stack;

use crate::*;
pub use any::{AnyValue, AnyValueDyn};
pub use owned::BoxedValue;
pub use primitive::PrimitiveValue;
pub use stack::StackValue;

pub type EvalValue<'a, 'eval> = VMResult<Conditional<StackValue<'a, 'eval>>>;

#[derive(PartialEq, Eq, Debug)]
pub enum Conditional<T> {
    Defined(T),
    Undefined,
}

impl<T> Conditional<T> {
    pub fn defined(&self) -> VMResult<&T> {
        match self {
            Conditional::Defined(value) => Ok(value),
            Conditional::Undefined => Err(VMError::ValueUndefined),
        }
    }
}
