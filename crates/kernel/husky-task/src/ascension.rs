use crate::*;
use husky_visual_protocol::IsVisualProtocol;

pub trait IsDevAscension {
    type Base: 'static;
    type LinkTime: IsLinkageTable;
    type Value;
    type RuntimeStorage: Default;
    type RuntimeTaskSpecificConfig: Default;
    type VisualProtocol: IsVisualProtocol;
}
