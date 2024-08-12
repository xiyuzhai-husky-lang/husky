use crate::*;
use husky_entity_path::path::ItemPath;
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki::{Ki, KiDomain};
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface};
use husky_linket_impl::{
    eval_context::{DevEvalContext, IsDevRuntimeDyn},
    linket_impl::{
        IsLinketImpl, LinketImplKiControlFlow, LinketImplTrackedExceptedValue,
        LinketImplTrackedException,
    },
    pedestal::IsPedestalFull,
};
use husky_linktime::IsLinktime;
use husky_trace_protocol::{
    id::TraceId,
    protocol::{IsTraceProtocol, IsTraceProtocolFull},
    server::KiVisualCache,
};
use husky_value_interface::ki_control_flow::KiControlFlow;
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use std::{cell::Cell, convert::Infallible, thread::LocalKey};

pub trait IsDevsoul: 'static {
    type Pedestal: IsPedestalFull;
    type LinketImpl: IsLinketImpl<Pedestal = Self::Pedestal>;
    type Linktime: IsLinktime<LinketImpl = Self::LinketImpl>;
    type RuntimeStorage: IsRuntimeStorage<Self::LinketImpl>;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol<Pedestal = Self::Pedestal> + IsTraceProtocolFull;
    fn calc_figure(
        followed: Option<(
            TraceId,
            KiReprInterface,
            KiDomainReprInterface,
            &[ItemPathIdInterface],
        )>,
        accompanyings_except_followed: &[(TraceId, KiReprInterface, &[ItemPathIdInterface])],
        pedestal: <Self::TraceProtocol as IsTraceProtocol>::Caryatid,
        runtime: &dyn IsDevRuntimeDyn<Self::LinketImpl>,
        visual_synchrotron: &mut VisualSynchrotron,
        ki_visual_cache: &mut KiVisualCache<Self::Pedestal>,
    ) -> <Self::TraceProtocol as IsTraceProtocol>::Figure;

    /// final
    fn get_ki_visual(
        ki_repr: KiReprInterface,
        runtime: &dyn IsDevRuntimeDyn<Self::LinketImpl>,
        visual_synchrotron: &mut VisualSynchrotron,
        ki_visual_cache: &mut KiVisualCache<Self::Pedestal>,
    ) -> Visual {
        let pedestal = todo!();
        ki_visual_cache.get_visual(ki_repr, pedestal, || {
            use husky_value_interface::IsValue;
            match runtime.eval_ki_repr_interface_dyn(ki_repr) {
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

pub trait IsRuntimeStorage<LinketImpl: IsLinketImpl>: Default + Send {
    fn get_or_try_init_val_value(
        &self,
        val_item_path_id_interface: ItemPathIdInterface,
        pedestal: LinketImpl::Pedestal,
        f: impl FnOnce() -> LinketImplKiControlFlow<LinketImpl>,
        db: &::salsa::Db,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn get_or_try_init_memo_field_value(
        &self,
        item_path_id_interface: ItemPathIdInterface,
        slf: &'static std::ffi::c_void,
        f: impl FnOnce(&'static std::ffi::c_void) -> LinketImplKiControlFlow<LinketImpl>,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn get_or_try_init_ki_value(
        &self,
        ki: Ki,
        pedestal: LinketImpl::Pedestal,
        f: impl FnOnce() -> LinketImplKiControlFlow<LinketImpl>,
        db: &::salsa::Db,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn get_or_try_init_generic_gn_value(
        &self,
        ki: Ki,
        pedestal: LinketImpl::Pedestal,
        f: impl FnOnce() -> LinketImplKiControlFlow<LinketImpl>,
        db: &::salsa::Db,
    ) -> LinketImplKiControlFlow<LinketImpl>;

    fn get_or_try_init_ki_domain_value(
        &self,
        ki_domain: KiDomain,
        pedestal: LinketImpl::Pedestal,
        f: impl FnOnce() -> KiControlFlow<(), Infallible, LinketImplTrackedException<LinketImpl>>,
        db: &::salsa::Db,
    ) -> KiControlFlow<(), Infallible, LinketImplTrackedException<LinketImpl>>;
}

pub type DevEvalContextLocalKey<LinketImpl> =
    LocalKey<Cell<std::option::Option<DevEvalContext<LinketImpl>>>>;
