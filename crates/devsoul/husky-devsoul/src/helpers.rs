use husky_devsoul_interface::{IsLinkageImpl, LinkageImplKiControlFlow};

use crate::*;

pub type DevsoulValue<Devsoul> = <<Devsoul as IsDevsoul>::LinkageImpl as IsLinkageImpl>::Value;
pub type DevsoulException<Devsoul> =
    <<Devsoul as IsDevsoul>::LinkageImpl as IsLinkageImpl>::Exception;
pub type DevsoulValueResult<Devsoul> =
    LinkageImplKiControlFlow<<Devsoul as IsDevsoul>::LinkageImpl>;
pub type DevsoulKiControlFlow<Devsoul, C = DevsoulValue<Devsoul>> =
    LinkageImplKiControlFlow<<Devsoul as IsDevsoul>::LinkageImpl, C>;
