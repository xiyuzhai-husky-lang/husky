use crate::*;
use husky_linket_impl::{
    linket_impl::{IsLinketImpl, LinketImplKiControlFlow, LinketImplTrackedException},
    pedestal::IsPedestal,
};

pub type DevsoulValue<Devsoul> = <<Devsoul as IsDevsoul>::LinketImpl as IsLinketImpl>::Value;
pub type DevsoulStaticVarId<Devsoul> =
    <<<Devsoul as IsDevsoul>::LinketImpl as IsLinketImpl>::Pedestal as IsPedestal>::StaticVarId;
pub type DevsoulTrackedException<Devsoul> =
    LinketImplTrackedException<<Devsoul as IsDevsoul>::LinketImpl>;
pub type DevsoulValueResult<Devsoul> = LinketImplKiControlFlow<<Devsoul as IsDevsoul>::LinketImpl>;
pub type DevsoulKiControlFlow<Devsoul, C = DevsoulValue<Devsoul>> =
    LinketImplKiControlFlow<<Devsoul as IsDevsoul>::LinketImpl, C>;
