#![feature(const_trait_impl)]
pub mod __root;
mod config;
mod control;
mod entity;
mod error;
mod frame;
mod history;
mod instruction;
mod interpreter;
mod loop_kind;
mod mode;
mod mutation;
mod signature;
mod snapshot;
mod stack;
mod stack_idx;

pub use config::*;
pub use control::{ControlSnapshot, VMControl};
pub use entity::*;
pub use error::*;
pub use frame::{FrameKind, LoopFrameData};
pub use history::{History, HistoryEntry};
pub use husky_any::*;
pub use husky_vm_binding::Binding;
pub use husky_vm_interface::*;
pub use instruction::*;
pub use interpreter::{Interpreter, InterpreterQueryGroup};
pub use loop_kind::VMLoopKind;
pub use mode::Mode;
pub use mutation::*;
pub use signature::*;
pub use snapshot::StackSnapshot;
pub use stack::*;
pub use stack_idx::*;

use husky_ethereal_term::EtherealTerm;
use husky_loop_syntax::*;

use husky_coword::Ident;
use std::sync::Arc;

pub fn eval_fast<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    opt_ctx: Option<&'temp dyn __EvalContext<'eval>>,
    opt_instrn_sheet: Option<&InstructionSheet>,
    opt_linkage: Option<__Linkage>,
    args: impl Iterator<Item = __VMResult<__Register<'eval>>>, // including this value
    _kwargs: impl Iterator<Item = (Ident, __VMResult<__Register<'eval>>)>,
    nargs: u8,
    vm_config: &'temp VMConfig,
) -> __VMResult<__Register<'eval>> {
    let mut interpreter = Interpreter::try_new(db, opt_ctx, args, vm_config)?;
    if let Some(linkage) = opt_linkage {
        interpreter.eval_linkage(linkage, nargs)
    } else {
        interpreter.eval_instructions(opt_instrn_sheet.as_ref().unwrap(), Mode::Fast)
    }
}

pub fn exec_debug<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    opt_ctx: Option<&'temp dyn __EvalContext<'eval>>,
    sheet: &InstructionSheet,
    stack: VMStack<'eval>,
    vm_config: &'temp VMConfig,
) -> Arc<History<'eval>> {
    let mut interpreter = Interpreter::from_stack(db, opt_ctx, stack, vm_config);
    interpreter.exec_all(sheet, Mode::TrackHistory);
    Arc::new(interpreter.history)
}

pub fn exec_loop_debug<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    loop_kind: VMLoopKind,
    sheet: &InstructionSheet,
    stack_snapshot: &StackSnapshot<'eval>,
    vm_config: &'temp VMConfig,
) -> Vec<LoopFrameData<'eval>> {
    let mut interpreter = Interpreter::from_stack(db, opt_ctx, stack_snapshot.into(), vm_config);
    interpreter.exec_loop_tracking_frame(loop_kind, &sheet);
    interpreter.frames
}
