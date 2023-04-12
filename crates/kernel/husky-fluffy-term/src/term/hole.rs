use super::*;

/// source
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
#[enum_class::from_variants]
pub enum HoleSource {
    Expr(ExprIdx),
    Expectation(FluffyTermExpectationIdx),
}

impl HoleSource {
    pub fn expr_idx(self) -> ExprIdx {
        todo!()
        // self.expr_idx
    }
}

/// kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HoleKind {
    UnspecifiedIntegerType,
    UnspecifiedFloatType,
}

impl HollowTerm {
    pub(crate) fn new_hole(
        hollow_terms: &mut HollowTerms,
        src: impl Into<HoleSource>,
        hole_kind: HoleKind,
    ) -> Self {
        hollow_terms.alloc_new(HollowTermData::Hole(src.into(), hole_kind))
    }
}
