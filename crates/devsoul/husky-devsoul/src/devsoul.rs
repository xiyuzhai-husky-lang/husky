use crate::*;
use husky_entity_path::path::ItemPath;
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki::{Ki, KiDomain};
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface};
use husky_linket_impl::{
    dev_eval_context::{DevEvalContext, IsDevRuntimeInterfaceDyn},
    linket_impl::{
        IsLinketImpl, LinketImplKiControlFlow, LinketImplTrackedExceptedValue,
        LinketImplTrackedException,
    },
    pedestal::IsPedestalFull,
};
use husky_linktime::IsLinktime;
use husky_trace_protocol::{
    protocol::{IsTraceProtocol, IsTraceProtocolFull},
    server::TraceVisualCache,
    trace_id::TraceId,
};
use husky_value::ki_control_flow::KiControlFlow;
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use std::{cell::Cell, convert::Infallible, thread::LocalKey};

pub trait IsDevsoul: 'static {
    type Pedestal: IsPedestalFull;
    type LinketImpl: IsLinketImpl<Pedestal = Self::Pedestal>;
    type Linktime: IsLinktime<LinketImpl = Self::LinketImpl>;
    type RuntimeStorage: IsRuntimeStorage<Self::LinketImpl>;
    type RuntimeSpecificConfig: Default + Send;
    type TraceProtocol: IsTraceProtocol<Pedestal = Self::Pedestal> + IsTraceProtocolFull;
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
