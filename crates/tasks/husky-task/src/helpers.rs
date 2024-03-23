use husky_task_interface::{IsLinkageImpl, LinkageImplKiControlFlow};

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
pub type TaskError<Task> = <TaskLinkageImpl<Task> as IsLinkageImpl>::Exception;
pub type TaskTraceProtocol<Task> = <TaskDevAscension<Task> as IsDevAscension>::TraceProtocol;
pub type TaskValueResult<Task> = LinkageImplKiControlFlow<TaskLinkageImpl<Task>>;

pub type TaskKiControlFlow<Task, C = <TaskLinkageImpl<Task> as IsLinkageImpl>::Value> =
    LinkageImplKiControlFlow<TaskLinkageImpl<Task>, C>;
