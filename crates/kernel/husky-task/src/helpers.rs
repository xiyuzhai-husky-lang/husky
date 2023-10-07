use crate::*;

pub type DevLinkTime<Task: IsTask> = <Task::DevAscension as IsDevAscension>::LinkTime;
pub type DevRuntimeTaskSpecificConfig<Task: IsTask> =
    <Task::DevAscension as IsDevAscension>::RuntimeTaskSpecificConfig;
pub type DevRuntimeStorage<Task: IsTask> = <Task::DevAscension as IsDevAscension>::RuntimeStorage;
pub type DevAscensionBase<Task: IsTask> = <Task::DevAscension as IsDevAscension>::Base;
