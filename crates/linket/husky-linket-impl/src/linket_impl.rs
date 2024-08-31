use crate::LinketImplVmControlFlow;
use crate::{exception::TrackedException, *};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::{KiArgumentReprInterface, KiDomainReprInterface, KiReprInterface};
use husky_value_interface::{exception::IsException, ki_control_flow::KiControlFlow, IsValue};
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use pedestal::{IsPedestal, IsPedestalFull};
use serde::Serialize;
use static_var::StaticVarResult;
use std::num::Saturating;

pub type LinketImplStaticVarResult<LinketImpl, R> =
    StaticVarResult<<<LinketImpl as IsLinketImpl>::Pedestal as IsPedestal>::VarId, R>;

pub trait IsLinketImpl: Send + Copy + 'static {
    type Pedestal: IsPedestalFull;
    type Value: IsValue<Exception = Self::Exception>;
    type Exception: IsException;

    /// assumed that pedestal has already been
    fn eval_ki(
        self,
        ki_repr_interface: KiReprInterface,
        ki_domain_repr_interface: KiDomainReprInterface,
        arguments: &[KiArgumentReprInterface],
        ctx: DevEvalContext<Self>,
    ) -> LinketImplKiControlFlow<Self>;

    fn eval_vm(
        self,
        arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> LinketImplVmControlFlow<Self>;

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter;

    /// applies only for static var linkage
    fn init_item_path_id_interface(self, item_path_id_interface: ItemPathIdInterface);
    /// applies only for static var linkage
    fn static_var_id(self) -> <Self::Pedestal as IsPedestal>::VarId;
    fn with_var_id<R>(
        self,
        static_var_id: <Self::Pedestal as IsPedestal>::VarId,
        locked: &[ItemPathIdInterface],
        f: impl FnOnce() -> R,
    ) -> LinketImplStaticVarResult<Self, R>;
    /// applies only for static var linkage
    fn all_var_ids<'a>(
        self,
        locked: &'a [ItemPathIdInterface],
    ) -> Box<dyn Iterator<Item = <Self::Pedestal as IsPedestal>::VarId> + 'a>;
}

pub type LinketImplKiControlFlow<LinketImpl, C = <LinketImpl as IsLinketImpl>::Value> =
    KiControlFlow<C, <LinketImpl as IsLinketImpl>::Value, LinketImplTrackedException<LinketImpl>>;

pub type LinketImplTrackedException<LinketImpl> = TrackedException<
    <LinketImpl as IsLinketImpl>::Exception,
    <LinketImpl as IsLinketImpl>::Pedestal,
>;

pub type LinketImplTrackedExceptedValue<LinketImpl> =
    Result<<LinketImpl as IsLinketImpl>::Value, LinketImplTrackedException<LinketImpl>>;

pub type LinketImplTrackedExcepted<LinketImpl, T> =
    Result<T, LinketImplTrackedException<LinketImpl>>;

pub enum VmArgumentValue<LinketImpl: IsLinketImpl> {
    Simple(LinketImpl::Value),
    Variadic(Vec<LinketImpl::Value>),
}
