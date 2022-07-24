#![feature(const_trait_impl)]
pub mod __root;
mod call_form_value;
mod config;
mod context;
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

pub use call_form_value::*;
pub use config::*;
pub use context::*;
pub use control::{ControlSnapshot, VMControl};
pub use entity::*;
pub use error::*;
pub use frame::{FrameKind, LoopFrameData};
pub use history::{History, HistoryEntry};
pub use husky_any::*;
use husky_print_utils::p;
pub use husky_vm_runtime_error::*;
pub use instruction::*;
pub use interpreter::{Interpreter, InterpreterQueryGroup};
pub use linkage::*;
pub use loop_kind::{BoundaryKind, LoopStep, VMLoopKind};
pub use mode::Mode;
pub use mutation::*;
pub use signature::*;
pub use snapshot::StackSnapshot;
pub use stack::*;

use error::*;
use husky_entity_route::EntityRoutePtr;
use husky_trace_protocol::*;
use std::sync::Arc;
use word::CustomIdentifier;

pub fn eval_fast<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    opt_ctx: Option<&'temp dyn __EvalContext<'eval>>,
    opt_instrn_sheet: Option<&InstructionSheet>,
    opt_linkage: Option<__Linkage>,
    output_ty: EntityRoutePtr,
    args: impl Iterator<Item = __EvalResult<__TempValue<'temp, 'eval>>>, // including this value
    kwargs: impl Iterator<Item = (CustomIdentifier, __EvalResult<__TempValue<'temp, 'eval>>)>,
    nargs: u8,
    vm_config: &'temp VMConfig,
) -> __EvalValueResult<'eval> {
    let mut interpreter = Interpreter::try_new(db, opt_ctx, args, vm_config)?;
    if let Some(linkage) = opt_linkage {
        interpreter.eval_linkage(linkage, nargs, output_ty)
    } else {
        interpreter.eval_instructions(opt_instrn_sheet.as_ref().unwrap(), Mode::Fast)
    }
}

pub fn exec_debug<'temp, 'eval: 'temp>(
    db: &'temp dyn InterpreterQueryGroup,
    opt_ctx: Option<&'temp dyn __EvalContext<'eval>>,
    sheet: &InstructionSheet,
    stack: VMStack<'temp, 'eval>,
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
