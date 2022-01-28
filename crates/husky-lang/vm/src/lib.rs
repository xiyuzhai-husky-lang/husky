mod compiled;
mod contract;
mod error;
mod instruction;
mod stack;
mod value;

pub use compiled::Compiled;
pub use contract::InputContract;
pub use error::{VMError, VMResult};
pub use instruction::*;
pub use value::{
    AnyValue, AnyValueDyn, BoxedValue, Conditional, EvalValue, PrimitiveValue, StackValue,
};

use stack::VMStack;

pub enum ControlSignal<'a, 'eval: 'a> {
    Normal,
    Return(StackValue<'a, 'eval>),
    Break,
}

pub fn run<'a, 'eval: 'a>(instructions: &[Instruction]) -> VMResult<StackValue<'a, 'eval>> {
    let mut stack = VMStack::<'_, 'eval>::new(vec![]);
    match stack.exec_all(instructions)? {
        ControlSignal::Normal => todo!(),
        ControlSignal::Return(value) => Ok(value),
        ControlSignal::Break => todo!(),
    }
}
