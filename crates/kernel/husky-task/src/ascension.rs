use crate::*;

pub trait IsDevAscension {
    type Base: 'static;
    type LinkTime: IsLinkageTable;
    type Value;
    type RuntimeStorage: Default;
    type RuntimeTaskSpecificConfig: Default;
}
