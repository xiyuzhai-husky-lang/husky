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
mod signature;
mod snapshot;
mod stack;
mod ty;

use std::sync::Arc;

pub use compiled::{Compiled, MembVarAccessCompiled};
pub use contract::{InputContract, MembVarContract};
pub use control::{ControlSnapshot, VMControl};
pub use error::{VMError, VMResult};
pub use frame::{FrameKind, LoopFrameSnapshot};
pub use history::{History, HistoryEntry};
pub use instruction::*;
pub use interpreter::Interpreter;
pub use loop_kind::{BoundaryKind, LoopStep, VMLoopKind};
pub use mode::Mode;
pub use signature::*;
pub use snapshot::{StackSnapshot, StackValueSnapshot};
pub use stack::*;
pub use ty::*;

use error::*;

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
}

pub fn exec_debug<'stack, 'eval: 'stack>(
    values: impl Into<VMStack<'stack, 'eval>>,
    sheet: &InstructionSheet,
) -> Arc<History<'eval>> {
    let mut interpreter = Interpreter::new(values);
    interpreter.exec_all(&sheet.instructions, Mode::Debug);
    Arc::new(interpreter.history)
}

pub fn exec_loop_debug<'stack, 'eval: 'stack>(
    values: impl Into<VMStack<'stack, 'eval>>,
    loop_kind: VMLoopKind,
    sheet: &InstructionSheet,
) -> Vec<LoopFrameSnapshot<'eval>> {
    let mut interpreter = Interpreter::new(values);
    interpreter.exec_loop_debug(loop_kind, &sheet);
    interpreter.frames
}
