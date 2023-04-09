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
                ExpectationResolveProgress::Unresolved => true,
                ExpectationResolveProgress::Resolved(_) => false,
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
                ExpectationResolveProgress::Unresolved => true,
                ExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn alloc_rule(&mut self, rule: ExpectationEntry) -> FluffyTermExpectationIdx {
        self.arena.alloc_one(rule)
    }

    pub(crate) fn take_effect(
        &mut self,
        rule_idx: FluffyTermExpectationIdx,
        effect: FluffyTermExpectationEffect,
    ) -> Option<SmallVec<[FluffyTermResolveAction; 2]>> {
        self.arena
            .update(rule_idx, |rule| rule.set_resolved(effect.result));
        Some(effect.actions)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExpectationSource {
    Expr(ExprIdx),
    ExpectationResolve { parent: FluffyTermExpectationIdx },
}

impl ExpectationSource {
    pub fn expr_idx(self) -> ExprIdx {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectationEntry {
    src: ExpectationSource,
    expectee: FluffyTerm,
    data: ExpectationData,
    resolve_progress: ExpectationResolveProgress,
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

    pub fn resolve_progress(&self) -> &ExpectationResolveProgress {
        &self.resolve_progress
    }

    pub fn original_error(&self) -> Option<&OriginalFluffyTermExpectationError> {
        match self.resolve_progress {
            ExpectationResolveProgress::Resolved(Err(FluffyTermExpectationError::Original(
                ref e,
            ))) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn set_resolved(
        &mut self,
        result: FluffyTermExpectationResult<FluffyTermExpectationOutcome>,
    ) {
        self.resolve_progress = ExpectationResolveProgress::Resolved(result)
    }
}

impl FluffyTermRegion {
    pub fn add_expectation_rule(
        &mut self,
        src: ExpectationSource,
        expectee: FluffyTerm,
        expectation: impl Into<ExpectationData>,
    ) -> OptionFluffyTermExpectationIdx {
        self.expectations
            .alloc_rule(ExpectationEntry {
                src,
                expectee: expectee.into(),
                data: expectation.into(),
                resolve_progress: ExpectationResolveProgress::Unresolved,
            })
            .into()
    }
}
