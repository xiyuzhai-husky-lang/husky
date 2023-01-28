use super::*;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermIdx(usize);

pub(crate) struct UnresolvedTermTableEntry {
    unresolved_term: UnresolvedTerm,
    resolve_progress: LocalTerm,
}

#[derive(Default)]
pub(crate) struct UnresolvedTermTable {
    entries: Vec<UnresolvedTermTableEntry>,
    expectations: Arena<Expectation>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expectation {}
pub(crate) type ExpectationIdx = ArenaIdx<Expectation>;
pub(crate) type OptionExpectationIdx = OptionArenaIdx<Expectation>;

impl std::ops::Index<ExpectationIdx> for UnresolvedTermTable {
    type Output = Expectation;

    fn index(&self, index: ExpectationIdx) -> &Self::Output {
        &self.expectations[index]
    }
}

impl UnresolvedTermTable {
    pub(crate) fn unresolved_term(
        &self,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> &UnresolvedTerm {
        &self.entries[unresolved_term_idx.0].unresolved_term
    }

    pub(crate) fn intern_unresolved_term(
        &mut self,
        unresolved_term: UnresolvedTerm,
    ) -> UnresolvedTermIdx {
        let position = self
            .entries
            .iter()
            .position(|entry| entry.unresolved_term == unresolved_term);
        match position {
            Some(idx) => UnresolvedTermIdx(idx),
            None => {
                let idx = self.entries.len();
                self.entries.push(UnresolvedTermTableEntry {
                    unresolved_term,
                    resolve_progress: LocalTerm::Unresolved(UnresolvedTermIdx(idx)),
                });
                UnresolvedTermIdx(idx)
            }
        }
    }

    pub(crate) fn intern_expection(
        &mut self,
        expectation: Option<Expectation>,
    ) -> OptionExpectationIdx {
        match expectation {
            Some(expectation) => self.expectations.intern(expectation).into(),
            None => Default::default(),
        }
    }
}
