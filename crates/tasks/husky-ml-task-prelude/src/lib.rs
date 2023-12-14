use shifted_unsigned_int::ShiftedU32;
use std::cell::Cell;

pub struct SampleId(ShiftedU32);

pub type BasePoint = SampleId;

pub type DevEvalContext = husky_task_prelude::DevEvalContext<BasePoint>;

thread_local! {
    pub static DEV_EVAL_CONTEXT: Cell<Option<DevEvalContext>> = Cell::new(None);
}
