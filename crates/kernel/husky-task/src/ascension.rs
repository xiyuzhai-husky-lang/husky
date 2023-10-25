use crate::*;
use husky_vfs::VfsDb;
use husky_visual_protocol::IsVisualProtocol;

pub trait IsDevAscension {
    type Base: 'static;
    type ComptimeDb: Default + VfsDb;
    type Linktime: IsLinktime<Self::ComptimeDb>;
    type Value;
    type RuntimeStorage: Default + Send;
    type RuntimeTaskSpecificConfig: Default + Send;
    type VisualProtocol: IsVisualProtocol;
}

pub type DevAscension<Task: IsTask> = Task::DevAscension;

pub type DevComptimeDb<Task: IsTask> = <DevAscension<Task> as IsDevAscension>::ComptimeDb;
