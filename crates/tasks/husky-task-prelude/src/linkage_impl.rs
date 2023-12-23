use crate::{val_control_flow::ValControlFlow, val_repr::ValReprInterface};
use crate::{val_repr::ValArgumentReprInterface, DevEvalContext};

pub trait IsLinkageImpl: Send + Copy + 'static {
    type Pedestal: Copy + 'static;
    type Value: 'static;
    type Error;

    /// assumed that pedestal has already been
    fn eval(
        self,
        val_repr_interface: ValReprInterface,
        ctx: DevEvalContext<Self>,
        arguments: &[ValArgumentReprInterface],
    ) -> LinkageImplValueResult<Self>;
}

pub type LinkageImplValueResult<LinkageImpl: IsLinkageImpl> =
    Result<LinkageImpl::Value, LinkageImpl::Error>;
pub type LinkageImplValControlFlow<
    LinkageImpl: IsLinkageImpl,
    C = <LinkageImpl as IsLinkageImpl>::Value,
> = ValControlFlow<C, LinkageImpl::Value, LinkageImpl::Error>;
