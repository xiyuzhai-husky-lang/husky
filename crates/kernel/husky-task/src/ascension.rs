use crate::*;
use husky_visual_protocol::IsVisualProtocol;

pub trait IsDevAscension {
    type Base: 'static;
    type LinkTime: IsLinkTime;
    type Value;
    type RuntimeStorage: Default;
    type RuntimeTaskSpecificConfig: Default;
    type VisualProtocol: IsVisualProtocol;
}

pub type DevAscension<Task: IsTask> = Task::DevAscension;
