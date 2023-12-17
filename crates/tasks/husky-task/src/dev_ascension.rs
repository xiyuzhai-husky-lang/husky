use crate::*;
use husky_task_prelude::value::IsTaskValue;
use husky_task_prelude::{DevEvalContext, IsDevRuntime, IsDevRuntimeDyn};
use husky_trace_protocol::protocol::IsTraceProtocol;
use husky_vfs::VfsDb;
use std::{cell::Cell, thread::LocalKey};

pub trait IsDevAscension {
    type BasePoint: Copy + 'static;
    type Linktime: IsLinktime<BasePoint = Self::BasePoint>;
    type Value: IsTaskValue;
    type RuntimeStorage: Default + Send;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol;
    fn dev_eval_context_local_key() -> &'static LocalDevEvalContext<Self::BasePoint>;
}

pub type LocalDevEvalContext<BasePoint> =
    LocalKey<Cell<std::option::Option<DevEvalContext<BasePoint>>>>;

pub fn dev_eval_context<DevAscension: IsDevAscension>() -> DevEvalContext<DevAscension::BasePoint> {
    DevAscension::dev_eval_context_local_key()
        .get()
        .expect("`DEV_EVAL_CONTEXT` not set")
}

pub fn with_dev_eval_context<DevAscension: IsDevAscension, R>(
    ctx: DevEvalContext<DevAscension::BasePoint>,
    f: impl FnOnce() -> R,
) -> R {
    let local_key = DevAscension::dev_eval_context_local_key();
    let old = local_key.replace(Some(ctx));
    let r = f();
    local_key.set(old);
    r
}

pub fn with_runtime_and_base_point<
    DevAscension: IsDevAscension,
    Runtime: IsDevRuntime<DevAscension::BasePoint>,
    R,
>(
    runtime: &Runtime,
    base_point: DevAscension::BasePoint,
    f: impl FnOnce() -> R,
) -> R {
    let local_dev_eval_context = DevAscension::dev_eval_context_local_key();
    with_dev_eval_context::<DevAscension, _>(
        DevEvalContext::new(
            unsafe { runtime.cast_to_static_self_static_ref() },
            base_point,
        ),
        f,
    )
}
