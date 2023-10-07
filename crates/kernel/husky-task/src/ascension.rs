use crate::*;
use husky_visual_protocol::IsVisualProtocol;

pub trait IsDevAscension {
    type Base: 'static;
    type LinkageTable: IsLinkTime;
    type Value;
    type RuntimeStorage: Default;
    type RuntimeTaskSpecificConfig: Default;
    type VisualProtocol: IsVisualProtocol;
}
