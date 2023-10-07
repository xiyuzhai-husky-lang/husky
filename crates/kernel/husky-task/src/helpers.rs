use crate::*;

pub type DevLinkageTable<Task: IsTask> = <Task::DevAscension as IsDevAscension>::LinkageTable;
pub type DevRuntimeTaskSpecificConfig<Task: IsTask> =
    <Task::DevAscension as IsDevAscension>::RuntimeTaskSpecificConfig;
pub type DevRuntimeStorage<Task: IsTask> = <Task::DevAscension as IsDevAscension>::RuntimeStorage;
pub type DevAscensionBase<Task: IsTask> = <Task::DevAscension as IsDevAscension>::Base;
