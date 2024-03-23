use crate::*;
use husky_ki::Ki;
use husky_task_interface::{
    ki_control_flow::KiControlFlow,
    ki_repr::{KiReprInterface, ValDomainReprInterface},
    pedestal::IsPedestalFull,
    DevEvalContext, IsDevRuntime,
};
use husky_task_interface::{
    IsLinkageImpl, LinkageImplKiControlFlow, TaskIngredientIndex, TaskJarIndex,
};
use husky_trace_protocol::{
    id::TraceId,
    protocol::{IsTraceProtocol, IsTraceProtocolFull},
    server::ValVisualCache,
};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};

use std::{cell::Cell, thread::LocalKey};

pub trait IsDevAscension {
    type Pedestal: IsPedestalFull;
    type LinkageImpl: IsLinkageImpl<Pedestal = Self::Pedestal>;
    type Linktime: IsLinktime<LinkageImpl = Self::LinkageImpl>;
    type RuntimeStorage: IsRuntimeStorage<Self::LinkageImpl>;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol<Pedestal = Self::Pedestal> + IsTraceProtocolFull;
    fn calc_figure<DevRuntime: IsDevRuntime<Self::LinkageImpl>>(
        followed: Option<(TraceId, KiReprInterface, ValDomainReprInterface)>,
        accompanyings_except_followed: &[(TraceId, KiReprInterface)],
        pedestal: Self::Pedestal,
        runtime: &DevRuntime,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure;
    fn dev_eval_context_local_key() -> &'static DevEvalContextLocalKey<Self::LinkageImpl>;

    /// final
    #[track_caller]
    fn dev_eval_context() -> DevEvalContext<Self::LinkageImpl> {
        Self::dev_eval_context_local_key().get().unwrap()
    }

    /// final
    fn get_val_visual<DevRuntime: IsDevRuntime<Self::LinkageImpl>>(
        ki_repr: KiReprInterface,
        pedestal: Self::Pedestal,
        runtime: &DevRuntime,
        visual_synchrotron: &mut VisualSynchrotron,
        val_visual_cache: &mut ValVisualCache<Self::Pedestal>,
    ) -> Visual {
        val_visual_cache.get_visual(ki_repr, pedestal, || {
            use husky_value_interface::IsValue;
            match runtime.eval_ki_repr_interface_at_pedestal(ki_repr, pedestal) {
                KiControlFlow::Continue(value) => value.visualize(visual_synchrotron),
                KiControlFlow::LoopContinue => todo!(),
                KiControlFlow::LoopExit(_) => todo!(),
                KiControlFlow::Return(_) => todo!(),
                KiControlFlow::Undefined => todo!(),
                KiControlFlow::Throw(_) => todo!(),
            }
        })
    }
}

pub trait IsRuntimeStorage<LinkageImpl: IsLinkageImpl>: Default + Send {
    // todo: consider caching policy
    fn get_or_try_init_val_value(
        &self,
        val: Ki,
        pedestal: LinkageImpl::Pedestal,
        f: impl FnOnce() -> LinkageImplKiControlFlow<LinkageImpl>,
        db: &::salsa::Db,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn get_or_try_init_memo_field_value(
        &self,
        jar_index: TaskJarIndex,
        ingredient_index: TaskIngredientIndex,
        pedestal: LinkageImpl::Pedestal,
        slf: &'static std::ffi::c_void,
        f: impl FnOnce(&'static std::ffi::c_void) -> LinkageImplKiControlFlow<LinkageImpl>,
    ) -> LinkageImplKiControlFlow<LinkageImpl>;

    fn debug_drop(self);
}

pub type DevEvalContextLocalKey<LinkageImpl> =
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
