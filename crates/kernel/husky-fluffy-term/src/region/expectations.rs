use super::*;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Expectations {
    arena: Arena<ExpectationEntry>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<ExpectationIdx> for Expectations {
    type Output = ExpectationEntry;

    fn index(&self, index: ExpectationIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl Expectations {
    pub(crate) fn unresolved_expectation_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut ExpectationEntry> {
        self.arena
            .iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|entry| match entry.meta.resolve_progress() {
                ExpectationProgress::Intact | ExpectationProgress::Holed => true,
                ExpectationProgress::Resolved(_) => false,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &ExpectationEntry> {
        self.arena.iter()
    }

    pub(super) fn alloc_expectation(&mut self, entry: ExpectationEntry) -> ExpectationIdx {
        self.arena.alloc_one(entry)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExpectationSource {
    expr_idx: ExprIdx,
    kind: ExpectationSourceKind,
}

impl ExpectationSource {
    pub fn new_expr(expr_idx: ExprIdx) -> Self {
        Self {
            expr_idx,
            kind: ExpectationSourceKind::Expr,
        }
    }

    pub(crate) fn child_src(self, idx: ExpectationIdx) -> Self {
        Self {
            expr_idx: self.expr_idx,
            kind: ExpectationSourceKind::Expectation(idx),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExpectationSourceKind {
    Expr,
    Expectation(ExpectationIdx),
}

impl ExpectationSource {
    pub fn expr_idx(self) -> ExprIdx {
        self.expr_idx
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectationEntry {
    expectation: Expectation,
    meta: ExpectationMeta,
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectationMeta {
    idx: ExpectationIdx,
    src: ExpectationSource,
    expectee: FluffyTerm,
    resolve_progress: ExpectationProgress,
}

impl ExpectationEntry {
    pub(crate) fn resolve(
        &mut self,
        db: &dyn FluffyTermDb,
        terms: &mut FluffyTerms,
    ) -> Option<ExpectationEffect> {
        self.expectation.resolve(db, &mut self.meta, terms)
    }

    #[inline]
    pub fn resolve_progress(&self) -> &ExpectationProgress {
        &self.meta.resolve_progress
    }

    #[inline]
    pub fn src(&self) -> ExpectationSource {
        self.meta.src
    }

    #[inline]
    pub fn original_error(&self) -> Option<&OriginalFluffyTermExpectationError> {
        match self.meta.resolve_progress {
            ExpectationProgress::Resolved(Err(FluffyTermExpectationError::Original(ref e))) => {
                Some(e)
            }
            _ => None,
        }
    }
}

impl ExpectationMeta {
    pub fn src(&self) -> ExpectationSource {
        self.src
    }

    pub(crate) fn child_src(&self) -> ExpectationSource {
        self.src.child_src(self.idx())
    }

    pub(crate) fn expectee(&self) -> FluffyTerm {
        self.expectee
    }

    pub(crate) fn resolve_progress(&self) -> &ExpectationProgress {
        &self.resolve_progress
    }

    pub(crate) fn idx(&self) -> ExpectationIdx {
        self.idx
    }

    pub(crate) fn set_holed(
        &mut self,
        hole: Hole,
        gen_hole_constraint: impl FnOnce(&mut Self) -> HoleConstraint,
    ) -> Option<ExpectationEffect> {
        match self.resolve_progress {
            ExpectationProgress::Holed => return None,
            ExpectationProgress::Resolved(_) => unreachable!(),
            ExpectationProgress::Intact => (),
        }
        self.resolve_progress = ExpectationProgress::Holed;
        Some(ExpectationEffect {
            subsequent_actions: smallvec![FluffyTermResolveAction::AddHoleConstraint {
                hole,
                hole_constraint: gen_hole_constraint(self)
            }],
        })
    }

    /// returns option for convenience
    pub(crate) fn set_ok(
        &mut self,
        outcome: impl Into<FluffyTermExpectationOutcome>,
        subsequent_actions: FluffyTermResolveActions,
    ) -> Option<ExpectationEffect> {
        #[cfg(test)]
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            _ => (),
        }
        self.resolve_progress = ExpectationProgress::Resolved(Ok(outcome.into()));
        Some(ExpectationEffect { subsequent_actions })
    }

    pub(crate) fn set_err(
        &mut self,
        e: impl Into<FluffyTermExpectationError>,
        subsequent_actions: FluffyTermResolveActions,
    ) -> Option<ExpectationEffect> {
        #[cfg(test)]
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            _ => (),
        }
        self.resolve_progress = ExpectationProgress::Resolved(Err(e.into()));
        Some(ExpectationEffect { subsequent_actions })
    }
}

#[derive(Default)]
pub struct ExpectationEffect {
    subsequent_actions: FluffyTermResolveActions,
}

impl ExpectationEffect {
    pub(crate) fn take_subsequent_actions(self) -> FluffyTermResolveActions {
        self.subsequent_actions
    }
}

impl FluffyTermRegion {
    pub fn add_expectation(
        &mut self,
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: impl Into<Expectation>,
    ) -> OptionFluffyTermExpectationIdx {
        let idx = unsafe { self.expectations.arena.next_idx() };
        self.expectations
            .alloc_expectation(ExpectationEntry {
                expectation: expectation.into(),
                meta: ExpectationMeta {
                    idx,
                    src,
                    expectee: expectee.into(),
                    resolve_progress: ExpectationProgress::Intact,
                },
            })
            .into()
    }
}
