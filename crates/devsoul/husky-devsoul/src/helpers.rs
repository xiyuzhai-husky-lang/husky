use husky_devsoul_interface::{pedestal::IsPedestal, IsLinkageImpl, LinkageImplKiControlFlow};

use crate::*;

pub type DevsoulValue<Devsoul> = <<Devsoul as IsDevsoul>::LinkageImpl as IsLinkageImpl>::Value;
pub type DevsoulStaticVarId<Devsoul> =
    <<<Devsoul as IsDevsoul>::LinkageImpl as IsLinkageImpl>::Pedestal as IsPedestal>::StaticVarId;
pub type DevsoulException<Devsoul> =
    <<Devsoul as IsDevsoul>::LinkageImpl as IsLinkageImpl>::Exception;
pub type DevsoulValueResult<Devsoul> =
    LinkageImplKiControlFlow<<Devsoul as IsDevsoul>::LinkageImpl>;
pub type DevsoulKiControlFlow<Devsoul, C = DevsoulValue<Devsoul>> =
    LinkageImplKiControlFlow<<Devsoul as IsDevsoul>::LinkageImpl, C>;
