#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LinkageInstantiation {}

impl LinkageInstantiation {
    pub(crate) fn new_ad_hoc() -> LinkageInstantiation {
        LinkageInstantiation {}
    }
}
