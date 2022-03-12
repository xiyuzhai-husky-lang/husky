mod compiled;
mod contract;
mod control;
mod error;
mod frame;
mod history;
mod instruction;
mod interpreter;
mod loop_kind;
mod mode;
mod snapshot;
mod stack;

use std::sync::Arc;

pub use compiled::Compiled;
pub use contract::Contract;
pub use control::{ControlSnapshot, VMControl};
use error::error;
pub use error::{VMError, VMResult};
pub use frame::{FrameKind, LoopFrameSnapshot};
pub use history::{History, HistoryEntry};
pub use instruction::*;
pub use interpreter::Interpreter;
pub use loop_kind::{BoundaryKind, LoopStep, VMLoopKind};
pub use mode::Mode;
pub use snapshot::{StackSnapshot, StackValueSnapshot};
pub use stack::{
    AnyValue, AnyValueDyn, BoxedValue, EvalResult, EvalValue, PrimitiveValue, StackIdx, StackValue,
    VMStack, VariableStack,
};

// fn run<'a, 'eval: 'a>(
//     value_iter: impl Iterator<Item = VMResult<StackValue<'a, 'eval>>>,
//     instructions: &[Instruction],
//     mode: Mode,
// ) -> EvalResult<'eval> {
//     let mut interpreter = Interpreter::<'_, 'eval>::try_new(value_iter)?;
//     match interpreter.exec_all(instructions, mode)? {
//         VMControl::None => todo!(),
//         VMControl::Return(result) => Ok(result),
//         VMControl::Break => todo!(),
//     }
// }

pub fn eval_fast<'a, 'eval: 'a>(
    iter: impl Iterator<Item = VMResult<StackValue<'a, 'eval>>>,
    sheet: &InstructionSheet,
    maybe_compiled: Option<()>,
) -> EvalResult<'eval> {
    let mut interpreter = Interpreter::try_new(iter)?;
    if let Some(compiled) = maybe_compiled {
        interpreter.exec_compiled(compiled)
    } else {
        interpreter.eval_instructions(&sheet.instructions, Mode::Fast)
    }
    // vm::VMControl::Normal | vm::VMControl::Break => panic!(),
    // vm::VMControl::Return(value) => Ok(Conditional::Defined(value)),
    // todo!()
}
// Vec<StackValue<'a, 'eval>>
// pub fn debug_eval<'stack, 'eval: 'stack>(
//     values: impl Into<VMStack<'stack, 'eval>>,
//     sheet: &InstructionSheet,
// ) -> (EvalResult<'eval>, Arc<History>) {
//     let mut interpreter = Interpreter::new(values, Mode::Debug);
//     let result = interpreter.eval_instructions(&sheet.instructions, Mode::Debug);
//     (result, Arc::new(interpreter.history))
// }

pub fn exec_debug<'stack, 'eval: 'stack>(
    values: impl Into<VMStack<'stack, 'eval>>,
    sheet: &InstructionSheet,
) -> Arc<History> {
    let mut interpreter = Interpreter::new(values);
    interpreter.exec_all(&sheet.instructions, Mode::Debug);
    Arc::new(interpreter.history)
}

pub fn exec_loop_debug<'stack, 'eval: 'stack>(
    values: impl Into<VMStack<'stack, 'eval>>,
    loop_kind: VMLoopKind,
    sheet: &InstructionSheet,
) -> Vec<LoopFrameSnapshot> {
    let mut interpreter = Interpreter::new(values);
    interpreter.exec_loop_debug(loop_kind, &sheet);
    interpreter.frames
}
