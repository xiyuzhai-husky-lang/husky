use crate::*;

pub type VisualProtocol<Task: IsTask> = <DevAscension<Task> as IsDevAscension>::VisualProtocol;
