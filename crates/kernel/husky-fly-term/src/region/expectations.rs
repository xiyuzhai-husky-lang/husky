use super::*;
use idx_arena::Arena;

#[salsa::debug_with_db]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Expectations {
    arena: Arena<FlyTermExpectationEntry>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<FlyTermExpectationIdx> for Expectations {
    type Output = FlyTermExpectationEntry;

    fn index(&self, index: FlyTermExpectationIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl Expectations {
    pub(crate) fn unresolved_expectation_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut FlyTermExpectationEntry> {
        self.arena
            .iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|entry| match entry.state.resolve_progress() {
                ExpectationProgress::Intact | ExpectationProgress::Holed => true,
                ExpectationProgress::Resolved(_) => false,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &FlyTermExpectationEntry> {
        self.arena.iter()
    }

    pub(super) fn alloc_expectation(
        &mut self,
        entry: FlyTermExpectationEntry,
    ) -> FlyTermExpectationIdx {
        self.arena.alloc_one(entry)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExpectationSource {
    syn_expr_idx: SynExprIdx,
    kind: ExpectationSourceKind,
}

impl ExpectationSource {
    pub fn new_expr(syn_expr_idx: SynExprIdx) -> Self {
        Self {
            syn_expr_idx,
            kind: ExpectationSourceKind::Expr,
        }
    }

    pub(crate) fn child_src(self, idx: FlyTermExpectationIdx) -> Self {
        Self {
            syn_expr_idx: self.syn_expr_idx,
            kind: ExpectationSourceKind::Expectation(idx),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExpectationSourceKind {
    Expr,
    Expectation(FlyTermExpectationIdx),
}

impl ExpectationSource {
    pub fn syn_expr_idx(self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct FlyTermExpectationEntry {
    expectation: Expectation,
    state: ExpectationState,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ExpectationState {
    idx: FlyTermExpectationIdx,
    src: ExpectationSource,
    expectee: FlyTerm,
    resolve_progress: ExpectationProgress,
}

impl FlyTermExpectationEntry {
    pub(crate) fn resolve(
        &mut self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
    ) -> AltOption<FlyTermEffect> {
        self.expectation.resolve(db, terms, &mut self.state)
    }

    #[inline]
    pub fn resolve_progress(&self) -> &ExpectationProgress {
        &self.state.resolve_progress
    }

    #[inline]
    pub fn src(&self) -> ExpectationSource {
        self.state.src
    }

    #[inline]
    pub fn original_error(&self) -> Option<&OriginalFlyTermExpectationError> {
        match self.state.resolve_progress {
            ExpectationProgress::Resolved(Err(FlyTermExpectationError::Original(ref e))) => Some(e),
            _ => None,
        }
    }
}

impl ExpectationState {
    pub fn src(&self) -> ExpectationSource {
        self.src
    }

    pub(crate) fn child_src(&self) -> ExpectationSource {
        self.src.child_src(self.idx())
    }

    pub(crate) fn expectee(&self) -> FlyTerm {
        self.expectee
    }

    pub(crate) fn resolve_progress(&self) -> &ExpectationProgress {
        &self.resolve_progress
    }

    pub(crate) fn idx(&self) -> FlyTermExpectationIdx {
        self.idx
    }

    pub(crate) fn set_holed(
        &mut self,
        hole: Hole,
        gen_hole_constraint: impl FnOnce(&mut Self) -> HoleConstraint,
    ) -> AltOption<FlyTermEffect> {
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            ExpectationProgress::Holed => return AltNone,
            ExpectationProgress::Intact => (),
        }
        self.resolve_progress = ExpectationProgress::Holed;
        AltSome(FlyTermEffect {
            subsequent_actions: smallvec![FlyTermResolveAction::AddHoleConstraint {
                hole,
                hole_constraint: gen_hole_constraint(self)
            }],
        })
    }

    /// returns option for convenience
    pub(crate) fn set_ok(
        &mut self,
        outcome: impl Into<ExpectationOutcome>,
        subsequent_actions: FlyTermResolveActions,
    ) -> AltOption<FlyTermEffect> {
        #[cfg(test)]
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            _ => (),
        }
        self.resolve_progress = ExpectationProgress::Resolved(Ok(outcome.into()));
        AltSome(FlyTermEffect { subsequent_actions })
    }

    pub(crate) fn set_err(
        &mut self,
        e: impl Into<FlyTermExpectationError>,
        subsequent_actions: FlyTermResolveActions,
    ) -> AltOption<FlyTermEffect> {
        #[cfg(test)]
        match self.resolve_progress {
            ExpectationProgress::Resolved(_) => unreachable!(),
            _ => (),
        }
        self.resolve_progress = ExpectationProgress::Resolved(Err(e.into()));
        AltSome(FlyTermEffect { subsequent_actions })
    }

    pub(crate) fn set_result<Outcome: Into<ExpectationOutcome>>(
        &mut self,
        result: FlyTermExpectationResult<Outcome>,
        subsequent_actions: FlyTermResolveActions,
    ) -> AltOption<FlyTermEffect> {
        match result {
            Ok(outcome) => self.set_ok(outcome, subsequent_actions),
            Err(e) => self.set_err(e, subsequent_actions),
        }
    }
}

#[derive(Debug, Default)]
pub struct FlyTermEffect {
    pub(crate) subsequent_actions: FlyTermResolveActions,
}

impl FlyTermEffect {
    pub(crate) fn take_subsequent_actions(self) -> FlyTermResolveActions {
        self.subsequent_actions
    }
}

impl FlyTermRegion {
    /// returns expectation idx and also the type after replacing implicit parameters with holes
    pub fn add_expectation<E: ExpectFlyTerm>(
        &mut self,
        src: ExpectationSource,
        expectee: FlyTerm,
        expectation: E,
        db: &::salsa::Db,
    ) -> (FlyTermExpectationIdx, FlyTerm) {
        let idx = unsafe { self.expectations.arena.next_idx() };
        let resolve_progress = E::initial_resolve_progress();
        (
            self.expectations
                .alloc_expectation(FlyTermExpectationEntry {
                    expectation: expectation.into(),
                    state: ExpectationState {
                        idx,
                        src,
                        expectee,
                        resolve_progress,
                    },
                }),
            expectee,
        )
    }
}
