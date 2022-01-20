mod any;
mod compiled;
mod contract;
mod error;
mod instruction;
mod stack;
mod value;

pub use any::{Any, HasRef};
pub use compiled::Compiled;
pub use contract::InputContract;
pub use error::{InterpretResult, InterpretStackError};
pub use instruction::*;
pub use value::{DurableValue, PrimitiveValue, StackValue};

use stack::VirtualStack;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlSignal {
    Normal,
    Return,
    Break,
}

pub fn interpret(instructions: &[Instruction]) -> InterpretResult<DurableValue> {
    todo!()
}
