use crate::IsLinktime;
use husky_linket_impl::{
    linket_impl::IsLinketImpl, LinketImplVmControlFlow, LinketImplVmControlFlowFrozen,
};

pub type LinktimeValue<Linktime> = <<Linktime as IsLinktime>::LinketImpl as IsLinketImpl>::Value;
pub type LinktimeVmControlFlow<Linktime> =
    LinketImplVmControlFlow<<Linktime as IsLinktime>::LinketImpl>;
pub type LinktimeVmControlFlowFrozen<Linktime> =
    LinketImplVmControlFlowFrozen<<Linktime as IsLinktime>::LinketImpl>;
