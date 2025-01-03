use super::*;
use idx_arena::ArenaIdx;

/// source
///
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum HoleSource {
    Expr(SynExprIdx),
    SemExpr(ArenaIdx<()>),
    Pattern(SynPatternIdx),
    Expectation(FlyTermExpectationIdx),
}

impl HoleSource {
    pub fn expr_idx(self) -> SynExprIdx {
        todo!()
        // self.expr_idx
    }
}

/// kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HoleKind {
    UnspecifiedIntegerType,
    UnspecifiedFloatType,
    ImplicitType,
    AnyOriginal,
    AnyDerived,
}

impl HolTerm {
    pub(crate) fn new(engine: &mut impl FlyTermEngineMut, data: HolTermData) -> Self {
        engine
            .fly_term_region_mut()
            .hollow_terms_mut()
            .alloc_new(data)
    }

    pub(crate) fn new_hole(
        engine: &mut impl FlyTermEngineMut,
        src: impl Into<HoleSource>,
        hole_kind: HoleKind,
    ) -> Self {
        Self::new(
            engine,
            HolTermData::Hole {
                hole_source: src.into(),
                hole_kind,
                fill: None,
                constraints: smallvec![],
            },
        )
    }
}
