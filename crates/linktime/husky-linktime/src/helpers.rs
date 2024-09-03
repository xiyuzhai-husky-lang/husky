use crate::IsLinktime;
use husky_linket_impl::{
    linket_impl::{IsLinketImpl, LinketImplThawedValue},
    LinketImplVmControlFlowFrozen, LinketImplVmControlFlowThawed,
};
use husky_value::IsValue;

pub type LinktimeValue<Linktime> = <<Linktime as IsLinktime>::LinketImpl as IsLinketImpl>::Value;
pub type LinktimeThawedValue<Linktime> =
    LinketImplThawedValue<<Linktime as IsLinktime>::LinketImpl>;
pub type LinktimeSlushValue<Linktime> =
    <<<Linktime as IsLinktime>::LinketImpl as IsLinketImpl>::Value as IsValue>::SlushValue;
pub type LinktimeVmControlFlow<Linktime> =
    LinketImplVmControlFlowThawed<<Linktime as IsLinktime>::LinketImpl>;
pub type LinktimeVmControlFlowFrozen<Linktime> =
    LinketImplVmControlFlowFrozen<<Linktime as IsLinktime>::LinketImpl>;
