use crate::{visual::IsVisualProtocol, *};

pub trait IsDevAscension {
    type Base: 'static;
    type LinkTime: IsLinkageTable;
    type Value;
    type RuntimeStorage: Default;
    type RuntimeTaskSpecificConfig: Default;
    type VisualProtocol: IsVisualProtocol;
}
