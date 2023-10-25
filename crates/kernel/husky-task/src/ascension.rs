use crate::*;
use husky_visual_protocol::IsVisualProtocol;

pub trait IsDevAscension {
    type Base: 'static;
    type LinkTime: IsLinkTime;
    type Value;
    type RuntimeStorage: Default + Send;
    type RuntimeTaskSpecificConfig: Default + Send;
    type VisualProtocol: IsVisualProtocol;
}

pub type DevAscension<Task: IsTask> = Task::DevAscension;
