mod compiled;
mod contract;
mod error;
mod instruction;
mod interpreter;
mod value;

pub use compiled::Compiled;
pub use contract::InputContract;
pub use error::{VMError, VMResult};
pub use instruction::*;
pub use interpreter::{BasicInterpreter, Interpreter};
pub use value::{
    AnyValue, AnyValueDyn, BoxedValue, Conditional, EvalValue, PrimitiveValue, StackValue,
};

pub enum ControlSignal<'a, 'eval: 'a> {
    Normal,
    Return(StackValue<'a, 'eval>),
    Break,
}

pub fn run<'a, 'eval: 'a>(instructions: &[Instruction]) -> VMResult<StackValue<'a, 'eval>> {
    let mut interpreter = BasicInterpreter::<'_, 'eval>::new(vec![]);
    match interpreter.exec_all(instructions)? {
        ControlSignal::Normal => todo!(),
        ControlSignal::Return(value) => Ok(value),
        ControlSignal::Break => todo!(),
    }
}
