use shifted_unsigned_int::ShiftedU32;
use std::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SampleId(ShiftedU32);

pub type BasePoint = SampleId;

pub type DevEvalContext = husky_task_prelude::DevEvalContext<BasePoint>;

thread_local! {
    pub static DEV_EVAL_CONTEXT: Cell<Option<DevEvalContext>> = Cell::new(None);
}

/// panics if dev eval context is empty
#[track_caller]
pub fn sample_id() -> SampleId {
    *DEV_EVAL_CONTEXT.get().unwrap().base_point()
}
