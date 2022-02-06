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

#[derive(PartialEq, Eq)]
pub enum Conditional<T> {
    Defined(T),
    Undefined,
}

impl<T> std::fmt::Debug for Conditional<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            Conditional::Defined(v) => v.fmt(f),
            Conditional::Undefined => f.write_str("undefined"),
        }
    }
}

impl<T> Conditional<T> {
    pub fn defined(self) -> VMResult<T> {
        match self {
            Conditional::Defined(value) => Ok(value),
            Conditional::Undefined => Err(VMError::ValueUndefined),
        }
    }

    pub fn defined_ref(&self) -> VMResult<&T> {
        match self {
            Conditional::Defined(value) => Ok(value),
            Conditional::Undefined => Err(VMError::ValueUndefined),
        }
    }
}
