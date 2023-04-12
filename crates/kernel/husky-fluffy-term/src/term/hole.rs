use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HoleRegistry {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HoleIdn {}

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
        todo!()
    }
}
