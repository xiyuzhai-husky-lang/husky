use husky_task_prelude::{IsLinkageImpl, LinkageImplValueResult};

use crate::*;

pub type TaskDevLinkTime<Task> = <<Task as IsTask>::DevAscension as IsDevAscension>::Linktime;
pub type TaskDevLinkageImpl<Task> =
    <<<Task as IsTask>::DevAscension as IsDevAscension>::Linktime as IsLinktime>::LinkageImpl;
pub type TaskDevRuntimeSpecificConfig<Task> =
    <<Task as IsTask>::DevAscension as IsDevAscension>::RuntimeSpecificConfig;
pub type DevRuntimeStorage<Task> =
    <<Task as IsTask>::DevAscension as IsDevAscension>::RuntimeStorage;
pub type TaskDevPedestal<Task> = <<<<Task as IsTask>::DevAscension as IsDevAscension> ::Linktime as IsLinktime>::LinkageImpl as IsLinkageImpl>::Pedestal;
pub type TaskDevAscension<Task> = <Task as IsTask>::DevAscension;
pub type TaskLinkageImpl<Task> =
    <<TaskDevAscension<Task> as IsDevAscension>::Linktime as IsLinktime>::LinkageImpl;
pub type TaskValue<Task> = <TaskLinkageImpl<Task> as IsLinkageImpl>::Value;
pub type TaskError<Task> = <TaskLinkageImpl<Task> as IsLinkageImpl>::Error;
pub type TaskTraceProtocol<Task> = <TaskDevAscension<Task> as IsDevAscension>::TraceProtocol;
pub type TaskValueResult<Task> = LinkageImplValueResult<TaskLinkageImpl<Task>>;
