use husky_task_interface::{IsLinkageImpl, LinkageImplKiControlFlow};

use crate::*;

pub type DevAscensionValue<DevAscension> =
    <<DevAscension as IsDevAscension>::LinkageImpl as IsLinkageImpl>::Value;
pub type DevAscensionException<DevAscension> =
    <<DevAscension as IsDevAscension>::LinkageImpl as IsLinkageImpl>::Exception;
pub type DevAscensionValueResult<DevAscension> =
    LinkageImplKiControlFlow<<DevAscension as IsDevAscension>::LinkageImpl>;
pub type DevAscensionKiControlFlow<DevAscension, C = DevAscensionValue<DevAscension>> =
    LinkageImplKiControlFlow<<DevAscension as IsDevAscension>::LinkageImpl, C>;
