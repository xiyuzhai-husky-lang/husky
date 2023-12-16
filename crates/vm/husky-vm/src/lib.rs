// #![feature(const_trait_impl)]
// pub mod __root;
// mod config;
// mod control;
// mod entity;
// mod error;
// mod frame;
// mod history;
// #[deprecated]
// mod instruction;
// mod interpreter;
// mod loop_kind;
// mod mode;
// mod mutation;
// mod signature;
// mod snapshot;
// mod stack;
// mod stack_idx;
// mod value;

// pub use self::config::*;
// pub use self::control::{ControlSnapshot, VMControl};
// pub use self::entity::*;
// pub use self::error::*;
// pub use self::frame::{FrameKind, LoopFrameData};
// pub use self::history::{History, HistoryEntry};
// pub use self::instruction::*;
// pub use self::interpreter::{Interpreter, InterpreterQueryGroup};
// pub use self::loop_kind::VMLoopKind;
// pub use self::mode::Mode;
// pub use self::mutation::*;
// pub use self::signature::*;
// pub use self::snapshot::StackSnapshot;
// pub use self::stack::*;
// pub use self::stack_idx::*;
// pub use husky_any::*;
// pub use husky_vm_binding::Binding;
// pub use husky_vm_interface::*;

// use husky_coword::Ident;
// use husky_ethereal_term::EtherealTerm;
// use husky_standard_value::*;
// use std::sync::Arc;

// pub fn eval_fast<'temp>(
//     db: &'temp dyn InterpreterQueryGroup,
//     opt_ctx: Option<&'temp dyn __EvalContext>,
//     opt_instrn_sheet: Option<&Instructions>,
//     opt_linkage: Option<__LinkageGroup>,
//     args: impl Iterator<Item = VMResult<RegularValue>>, // including this value
//     _kwargs: impl Iterator<Item = (Ident, VMResult<RegularValue>)>,
//     nargs: u8,
//     vm_config: &'temp VMConfig,
// ) -> VMResult<RegularValue> {
//     let mut interpreter = Interpreter::try_new(db, opt_ctx, args, vm_config)?;
//     if let Some(linkage) = opt_linkage {
//         interpreter.eval_linkage(linkage, nargs)
//     } else {
//         interpreter.eval_instructions(opt_instrn_sheet.as_ref().unwrap(), Mode::Fast)
//     }
// }

// pub fn exec_debug<'temp>(
//     db: &'temp dyn InterpreterQueryGroup,
//     opt_ctx: Option<&'temp dyn __EvalContext>,
//     sheet: &Instructions,
//     stack: VMStack,
//     vm_config: &'temp VMConfig,
// ) -> Arc<History> {
//     let mut interpreter = Interpreter::from_stack(db, opt_ctx, stack, vm_config);
//     interpreter.exec_all(sheet, Mode::TrackHistory);
//     Arc::new(interpreter.history)
// }

// pub fn exec_loop_debug<'temp>(
//     db: &'temp dyn InterpreterQueryGroup,
//     opt_ctx: Option<&dyn __EvalContext>,
//     loop_kind: VMLoopKind,
//     sheet: &Instructions,
//     stack_snapshot: &StackSnapshot,
//     vm_config: &'temp VMConfig,
// ) -> Vec<LoopFrameData> {
//     let mut interpreter = Interpreter::from_stack(db, opt_ctx, stack_snapshot.into(), vm_config);
//     interpreter.exec_loop_tracking_frame(loop_kind, &sheet);
//     interpreter.frames
// }
