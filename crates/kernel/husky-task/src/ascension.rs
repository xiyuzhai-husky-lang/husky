use crate::*;

pub trait IsDevAscension {
    type Base: 'static;
    type LinkTime: IsLinkageTable;
    type Value;
    type RuntimeStorage: Default + RefUnwindSafe + UnwindSafe;
    type RuntimeTaskSpecificConfig: Default + RefUnwindSafe + UnwindSafe;
}
