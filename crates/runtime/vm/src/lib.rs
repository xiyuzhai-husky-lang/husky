mod contract;
mod control;
mod entity;
mod enum_literal;
mod error;
mod frame;
mod history;
mod instruction;
mod interpreter;
mod linkage;
mod loop_kind;
mod mode;
mod signature;
mod snapshot;
mod stack;
mod ty;

pub use contract::{EagerContract, FieldContract, InputContract, LazyContract, OutputContract};
pub use control::{ControlSnapshot, VMControl};
pub use entity::*;
pub use enum_literal::{EnumLiteralValue, EnumLiteralValueDyn};
pub use error::{VMError, VMResult};
pub use frame::{FrameKind, LoopFrameSnapshot};
pub use history::{History, HistoryEntry};
pub use instruction::*;
pub use interpreter::{Interpreter, InterpreterQueryGroup};
pub use linkage::Linkage;
pub use loop_kind::{BoundaryKind, LoopStep, VMLoopKind};
pub use mode::Mode;
pub use signature::*;
pub use snapshot::{StackSnapshot, StackValueSnapshot};
pub use stack::*;
pub use ty::*;

use error::*;
use std::sync::Arc;

pub fn eval_fast<'stack, 'eval: 'stack>(
    db: &'stack dyn InterpreterQueryGroup,
    iter: impl Iterator<Item = VMResult<StackValue<'stack, 'eval>>>,
    sheet: &InstructionSheet,
    maybe_code: Option<Linkage>,
) -> EvalResult<'eval> {
    let mut interpreter = Interpreter::try_new(db, iter)?;
    if let Some(code) = maybe_code {
        interpreter.exec_code(code)
    } else {
        interpreter.eval_instructions(&sheet.instructions, Mode::Fast)
    }
}

pub fn exec_debug<'stack, 'eval: 'stack>(
    db: &'stack dyn InterpreterQueryGroup,
    values: impl Into<VMStack<'stack, 'eval>>,
    sheet: &InstructionSheet,
) -> Arc<History<'eval>> {
    let mut interpreter = Interpreter::new(db, values);
    interpreter.exec_all(&sheet.instructions, Mode::Debug);
    Arc::new(interpreter.history)
}

pub fn exec_loop_debug<'stack, 'eval: 'stack>(
    db: &'stack dyn InterpreterQueryGroup,
    values: impl Into<VMStack<'stack, 'eval>>,
    loop_kind: VMLoopKind,
    sheet: &InstructionSheet,
) -> Vec<LoopFrameSnapshot<'eval>> {
    let mut interpreter = Interpreter::new(db, values);
    interpreter.exec_loop_debug(loop_kind, &sheet);
    interpreter.frames
}
