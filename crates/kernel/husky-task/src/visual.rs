use crate::*;
use husky_visual_protocol::IsVisualProtocol;

pub type VisualProtocol<Task: IsTask> = <DevAscension<Task> as IsDevAscension>::VisualProtocol;

pub type VisualComponent<Task: IsTask> =
    <VisualProtocol<Task> as IsVisualProtocol>::VisualComponent;
