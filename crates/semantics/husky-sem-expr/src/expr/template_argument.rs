use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SemaTemplateArgumentList {
    langle: RegionalTokenIdx,
    arguments: SmallVec<[SemCommaListItem; 4]>,
    rangle: RegionalTokenIdx,
}

impl SemaTemplateArgumentList {
    pub(crate) fn new(
        langle: RegionalTokenIdx,
        arguments: SmallVec<[SemCommaListItem; 4]>,
        rangle: RegionalTokenIdx,
    ) -> Self {
        Self {
            langle,
            arguments,
            rangle,
        }
    }

    pub fn langle(&self) -> RegionalTokenIdx {
        self.langle
    }

    pub fn arguments(&self) -> &[SemCommaListItem] {
        &self.arguments
    }

    pub fn rangle(&self) -> RegionalTokenIdx {
        self.rangle
    }
}
