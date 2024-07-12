use husky_task_interface::{IsLinkageImpl, LinkageImplKiControlFlow};

use crate::*;

pub type DevendValue<Devend> = <<Devend as IsDevend>::LinkageImpl as IsLinkageImpl>::Value;
pub type DevendException<Devend> = <<Devend as IsDevend>::LinkageImpl as IsLinkageImpl>::Exception;
pub type DevendValueResult<Devend> = LinkageImplKiControlFlow<<Devend as IsDevend>::LinkageImpl>;
pub type DevendKiControlFlow<Devend, C = DevendValue<Devend>> =
    LinkageImplKiControlFlow<<Devend as IsDevend>::LinkageImpl, C>;
