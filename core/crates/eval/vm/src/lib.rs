#![feature(const_trait_impl)]
mod binding;
mod control;
mod entity;
mod error;
mod frame;
mod history;
mod instruction;
mod interpreter;
mod linkage;
mod loop_kind;
mod mode;
mod mutation;
mod signature;
mod snapshot;
mod stack;
mod value;
mod virtual_cyclic_slice;
mod virtual_struct;
mod virtual_vec;

pub use binding::*;
pub use control::{ControlSnapshot, VMControl};
pub use entity::*;
use entity_route::EntityRoutePtr;
pub use error::*;
pub use frame::{FrameKind, LoopFrameData};
pub use history::{History, HistoryEntry};
pub use instruction::*;
pub use interpreter::{Interpreter, InterpreterQueryGroup};
pub use linkage::*;
pub use loop_kind::{BoundaryKind, LoopStep, VMLoopKind};
pub use mode::Mode;
pub use mutation::*;
pub use signature::*;
pub use snapshot::{StackSnapshot, StackValueSnapshot};
pub use stack::*;
pub use value::*;
pub use virtual_cyclic_slice::*;
pub use virtual_struct::*;
pub use virtual_vec::*;

use error::*;
use husky_tracer_protocol::*;
use std::sync::Arc;
use word::CustomIdentifier;

pub fn eval_fast<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    opt_instrn_sheet: Option<&InstructionSheet>,
    opt_linkage: Option<Linkage>,
    output_ty: EntityRoutePtr,
    args: impl Iterator<Item = EvalResult<TempValue<'temp, 'eval>>>, // including this value
    kwargs: impl Iterator<Item = (CustomIdentifier, EvalResult<TempValue<'temp, 'eval>>)>,
    verbose: bool,
) -> EvalValueResult<'eval> {
    let mut interpreter = Interpreter::try_new(db, args, verbose)?;
    if let Some(linkage) = opt_linkage {
        interpreter.eval_linkage(linkage, output_ty)
    } else {
        interpreter.eval_instructions(opt_instrn_sheet.as_ref().unwrap(), Mode::Fast)
    }
}

pub fn exec_debug<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    sheet: &InstructionSheet,
    prestack: impl Into<VMStack<'temp, 'eval>>,
    verbose: bool,
) -> Arc<History<'eval>> {
    let mut interpreter = Interpreter::from_prestack(db, prestack, verbose);
    interpreter.exec_all(sheet, Mode::TrackHistory);
    Arc::new(interpreter.history)
}

pub fn exec_loop_debug<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    loop_kind: VMLoopKind,
    sheet: &InstructionSheet,
    stack_snapshot: &StackSnapshot<'eval>,
    verbose: bool,
) -> Vec<LoopFrameData<'eval>> {
    let mut interpreter = Interpreter::from_prestack(db, stack_snapshot, verbose);
    interpreter.exec_loop_tracking_frame(loop_kind, &sheet);
    interpreter.frames
}
