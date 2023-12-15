use crate::*;
use husky_task_prelude::value::IsTaskValue;
use husky_task_prelude::{DevEvalContext, IsDevRuntime, IsDevRuntimeDyn};
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_vfs::VfsDb;
use std::{cell::Cell, thread::LocalKey};

pub trait IsDevAscension {
    type BasePoint: 'static;
    type Linktime: IsLinktime;
    type Value: IsTaskValue;
    type RuntimeStorage: Default + Send;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol;
    fn local_dev_eval_context() -> &'static LocalDevEvalContext<Self::BasePoint>;
}

pub type LocalDevEvalContext<BasePoint> =
    LocalKey<Cell<std::option::Option<DevEvalContext<BasePoint>>>>;

pub fn with_eval_context<
    DevAscension: IsDevAscension,
    Runtime: IsDevRuntime<DevAscension::BasePoint>,
    R,
>(
    runtime: &Runtime,
    base_point: DevAscension::BasePoint,
    f: impl FnOnce() -> R,
) -> R {
    let local_dev_eval_context = DevAscension::local_dev_eval_context();
    let old = local_dev_eval_context.replace(Some(DevEvalContext::new(
        unsafe { runtime.cast_to_static_self_static_ref() },
        base_point,
    )));
    let r = f();
    local_dev_eval_context.set(old);
    r
}
