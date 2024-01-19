use crate::*;
use husky_task_interface::{val_repr::ValReprInterface, DevEvalContext, IsDevRuntime};
use husky_task_interface::{
    IsLinkageImpl, LinkageImplValControlFlow, TaskIngredientIndex, TaskJarIndex,
};
use husky_trace_protocol::{
    id::AccompanyingTraceIds,
    protocol::{IsTraceProtocol, IsTraceProtocolFull},
};
use husky_val::Val;

use std::{cell::Cell, thread::LocalKey};

pub trait IsDevAscension {
    type Pedestal: std::fmt::Debug + Copy + 'static;
    type LinkageImpl: IsLinkageImpl<Pedestal = Self::Pedestal>;
    type Linktime: IsLinktime<LinkageImpl = Self::LinkageImpl>;
    type RuntimeStorage: IsRuntimeStorage<Self::LinkageImpl>;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol<Pedestal = Self::Pedestal> + IsTraceProtocolFull;
    fn get_figure(
        followed_val_repr: Option<ValReprInterface>,
        accompanying_val_reprs: Vec<ValReprInterface>,
        pedestal: Self::Pedestal,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure;
    fn dev_eval_context_local_key() -> &'static LocalDevEvalContext<Self::LinkageImpl>;
}

pub trait IsRuntimeStorage<LinkageImpl: IsLinkageImpl>: Default + Send {
    // todo: consider caching policy
    fn get_or_try_init_val_value(
        &self,
        val: Val,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplValControlFlow<LinkageImpl>,
        db: &::salsa::Db,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    fn get_or_try_init_memoized_field_value(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        slf: &'static std::ffi::c_void,
        f: impl FnOnce(&'static std::ffi::c_void) -> LinkageImplValControlFlow<LinkageImpl>,
    ) -> LinkageImplValControlFlow<LinkageImpl>;

    fn debug_drop(self);
}

pub type LocalDevEvalContext<LinkageImpl> =
    LocalKey<Cell<std::option::Option<DevEvalContext<LinkageImpl>>>>;

pub fn dev_eval_context<DevAscension: IsDevAscension>(
) -> DevEvalContext<<DevAscension::Linktime as IsLinktime>::LinkageImpl> {
    DevAscension::dev_eval_context_local_key()
        .get()
        .expect("`DEV_EVAL_CONTEXT` not set")
}

pub fn with_dev_eval_context<DevAscension: IsDevAscension, R>(
    ctx: DevEvalContext<<DevAscension::Linktime as IsLinktime>::LinkageImpl>,
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
    Runtime: IsDevRuntime<<DevAscension::Linktime as IsLinktime>::LinkageImpl>,
    R,
>(
    runtime: &Runtime,
    base_point: <<DevAscension::Linktime as IsLinktime>::LinkageImpl as IsLinkageImpl>::Pedestal,
    f: impl FnOnce() -> R,
) -> R {
    let _local_dev_eval_context = DevAscension::dev_eval_context_local_key();
    with_dev_eval_context::<DevAscension, _>(
        DevEvalContext::new(
            unsafe { runtime.cast_to_static_self_static_ref() },
            base_point,
        ),
        f,
    )
}
