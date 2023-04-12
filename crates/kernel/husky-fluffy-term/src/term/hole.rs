use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HoleKind {
    UnspecifiedIntegerType,
    UnspecifiedFloatType,
}

impl HollowTerm {
    pub(crate) fn new_hole(
        hollow_terms: &mut HollowTerms,
        src: impl Into<HollowTermSource>,
        hole_kind: HoleKind,
    ) -> Self {
        hollow_terms.alloc_new(src.into(), HollowTermData::Hole(hole_kind))
    }
}
