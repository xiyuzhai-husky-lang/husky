pub use husky_ml_task_prelude_macros::*;

use shifted_unsigned_int::ShiftedU32;
use std::{cell::Cell, thread::LocalKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SampleId(ShiftedU32);

impl SampleId {
    pub fn from_index(index: usize) -> Self {
        Self(index.into())
    }

    pub fn index(self) -> usize {
        self.0.into()
    }
}

/// panics if dev eval context is empty
#[track_caller]
pub fn sample_id() -> SampleId {
    *DEV_EVAL_CONTEXT.get().unwrap().base_point()
}

#[test]
fn sample_id_size_works() {
    assert_eq!(std::mem::size_of::<SampleId>(), std::mem::size_of::<u32>());
    assert_eq!(
        std::mem::size_of::<Option<SampleId>>(),
        std::mem::size_of::<u32>()
    )
}

pub type BasePoint = SampleId;

pub type DevEvalContext = husky_task_prelude::DevEvalContext<BasePoint>;

thread_local! {
    pub static DEV_EVAL_CONTEXT: Cell<std::option::Option<DevEvalContext>> = Cell::new(None);
}

pub fn dev_eval_context() -> DevEvalContext {
    DEV_EVAL_CONTEXT.get().expect("`DEV_EVAL_CONTEXT` not set")
}

pub fn with_dev_eval_context<R>(ctx: DevEvalContext, f: impl FnOnce() -> R) -> R {
    let old = DEV_EVAL_CONTEXT.replace(Some(ctx));
    let r = f();
    DEV_EVAL_CONTEXT.set(old);
    r
}
