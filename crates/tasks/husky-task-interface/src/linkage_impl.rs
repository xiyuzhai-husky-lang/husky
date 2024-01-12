use crate::{val_control_flow::ValControlFlow, val_repr::ValReprInterface, value::IsValue};
use crate::{val_repr::ValArgumentReprInterface, DevEvalContext};
use serde::Serialize;
use serde_impl::IsSerdeImpl;

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Pedestal: std::fmt::Debug + Copy + 'static;
    type Value: IsValue;
    type Error: std::fmt::Debug + Serialize;

    /// assumed that pedestal has already been
    fn eval(
        self,
        val_repr_interface: ValReprInterface,
        ctx: DevEvalContext<Self>,
        arguments: &[ValArgumentReprInterface],
    ) -> LinkageImplValControlFlow<Self>;

    fn enum_u8_to_json_value(
        self,
    ) -> fn(u8) -> <<Self::Value as IsValue>::SerdeImpl as IsSerdeImpl>::Value;
}

pub type LinkageImplValControlFlow<LinkageImpl, C = <LinkageImpl as IsLinkageImpl>::Value> =
    ValControlFlow<C, <LinkageImpl as IsLinkageImpl>::Value, <LinkageImpl as IsLinkageImpl>::Error>;
