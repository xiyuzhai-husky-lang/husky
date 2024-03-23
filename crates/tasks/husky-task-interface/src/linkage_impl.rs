use crate::*;
use husky_value_protocol::presentation::EnumU8ValuePresenter;
use serde::Serialize;

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Pedestal: std::fmt::Debug + Copy + 'static;
    type Value: IsValue;
    type Exception: std::fmt::Debug + Serialize;

    /// assumed that pedestal has already been
    fn eval_ki(
        self,
        ki_repr_interface: KiReprInterface,
        ctx: DevEvalContext<Self>,
        arguments: &[KiArgumentReprInterface],
    ) -> LinkageImplKiControlFlow<Self>;

    fn enum_u8_value_presenter(self) -> EnumU8ValuePresenter;
}

pub type LinkageImplKiControlFlow<LinkageImpl, C = <LinkageImpl as IsLinkageImpl>::Value> =
    KiControlFlow<
        C,
        <LinkageImpl as IsLinkageImpl>::Value,
        <LinkageImpl as IsLinkageImpl>::Exception,
    >;
