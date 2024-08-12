use crate::LinketImplVmControlFlow;
use crate::{exception::TrackedException, *};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::{KiArgumentReprInterface, KiDomainReprInterface, KiReprInterface};
use husky_value_interface::{exception::IsException, ki_control_flow::KiControlFlow, IsValue};
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use pedestal::{IsPedestal, IsPedestalFull};
use serde::Serialize;
use std::num::Saturating;

pub trait IsLinketImpl: Send + Copy + 'static {
    type Pedestal: IsPedestalFull;
    type Value: IsValue<Exception = Self::Exception>;
    type Exception: IsException;

    ///
    fn init_item_path_id_interface(self, item_path_id_interface: ItemPathIdInterface);

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

    fn get_static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId;
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
