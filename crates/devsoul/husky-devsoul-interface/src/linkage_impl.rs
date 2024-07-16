use std::num::Saturating;

use self::vm_control_flow::LinkageImplVmControlFlow;
use crate::*;
use husky_value_protocol::presentation::EnumUnitValuePresenter;
use pedestal::{IsPedestal, IsPedestalFull};
use serde::Serialize;

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Pedestal: IsPedestalFull;
    type Value: IsValue;
    type Exception: std::fmt::Debug + Serialize;

    /// assumed that pedestal has already been
    fn eval_ki(
        self,
        ki_repr_interface: KiReprInterface,
        ctx: DevEvalContext<Self>,
        arguments: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<Self>;

    fn eval_vm(
        self,
        arguments: Vec<VmArgumentValue<Self>>,
        db: &dyn std::any::Any,
    ) -> LinkageImplVmControlFlow<Self>;

    fn enum_index_value_presenter(self) -> EnumUnitValuePresenter;

    fn get_static_var_id(self) -> <Self::Pedestal as IsPedestal>::StaticVarId;
}

pub type LinkageImplKiControlFlow<LinkageImpl, C = <LinkageImpl as IsLinkageImpl>::Value> =
    KiControlFlow<
        C,
        <LinkageImpl as IsLinkageImpl>::Value,
        <LinkageImpl as IsLinkageImpl>::Exception,
    >;

pub enum VmArgumentValue<LinkageImpl: IsLinkageImpl> {
    Simple(LinkageImpl::Value),
    Variadic(Vec<LinkageImpl::Value>),
}
