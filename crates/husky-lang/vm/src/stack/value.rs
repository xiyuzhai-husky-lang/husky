mod any;
mod eval;
mod owned;
mod primitive;
mod stack;

pub use any::{AnyValue, AnyValueDyn};
pub use eval::{EvalResult, EvalValue};
pub use owned::BoxedValue;
pub use primitive::PrimitiveValue;
pub use stack::StackValue;
