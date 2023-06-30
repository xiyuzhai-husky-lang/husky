use super::*;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Expectations {
    arena: Arena<ExpectationEntry>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<FluffyTermExpectationIdx> for Expectations {
    type Output = ExpectationEntry;

    fn index(&self, index: FluffyTermExpectationIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl Expectations {
    pub(crate) fn unresolved_rule_iter(
        &self,
    ) -> impl Iterator<Item = (FluffyTermExpectationIdx, &ExpectationEntry)> {
        self.arena
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                ExpectationProgress::Intact | ExpectationProgress::Holed => true,
                ExpectationProgress::Resolved(_) => false,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &ExpectationEntry> {
        self.arena.iter()
    }

    pub(crate) fn unresolved_indexed_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (FluffyTermExpectationIdx, &mut ExpectationEntry)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                ExpectationProgress::Intact | ExpectationProgress::Holed => true,
                ExpectationProgress::Resolved(_) => false,
            })
    }

    pub(super) fn alloc_expectation(
        &mut self,
        entry: ExpectationEntry,
    ) -> FluffyTermExpectationIdx {
        self.arena.alloc_one(entry)
    }

    pub(crate) fn take_effect(
        &mut self,
        expectation_idx: FluffyTermExpectationIdx,
        effect: FluffyTermExpectationEffect,
    ) -> Option<SmallVec<[FluffyTermResolveAction; 2]>> {
        self.arena.update(expectation_idx, |expectation_entry| {
            expectation_entry.set_resolved(effect.result)
        });
        Some(effect.actions)
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

    pub(crate) fn child_src(self, idx: FluffyTermExpectationIdx) -> Self {
        Self {
            expr_idx: self.expr_idx,
            kind: ExpectationSourceKind::Expectation(idx),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExpectationSourceKind {
    Expr,
    Expectation(FluffyTermExpectationIdx),
}

impl ExpectationSource {
    pub fn expr_idx(self) -> ExprIdx {
        self.expr_idx
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectationEntry {
    src: ExpectationSource,
    expectee: FluffyTerm,
    data: ExpectationData,
    resolve_progress: ExpectationProgress,
}

impl ExpectationEntry {
    pub fn src(&self) -> ExpectationSource {
        self.src
    }

    pub(crate) fn expectee(&self) -> FluffyTerm {
        self.expectee
    }

    pub(crate) fn data(&self) -> &ExpectationData {
        &self.data
    }

    pub fn resolve_progress(&self) -> &ExpectationProgress {
        &self.resolve_progress
    }

    pub fn original_error(&self) -> Option<&OriginalFluffyTermExpectationError> {
        match self.resolve_progress {
            ExpectationProgress::Resolved(Err(FluffyTermExpectationError::Original(ref e))) => {
                Some(e)
            }
            _ => None,
        }
    }

    pub(crate) fn set_resolved(
        &mut self,
        result: FluffyTermExpectationResult<FluffyTermExpectationOutcome>,
    ) {
        self.resolve_progress = ExpectationProgress::Resolved(result)
    }
}

impl FluffyTermRegion {
    pub fn add_expectation(
        &mut self,
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: impl Into<ExpectationData>,
    ) -> OptionFluffyTermExpectationIdx {
        self.expectations
            .alloc_expectation(ExpectationEntry {
                src,
                expectee: expectee.into(),
                data: expectation.into(),
                resolve_progress: ExpectationProgress::Intact,
            })
            .into()
    }
}
