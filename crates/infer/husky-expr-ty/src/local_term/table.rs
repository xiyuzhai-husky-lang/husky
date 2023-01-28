use super::*;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermIdx(usize);

pub(crate) struct UnresolvedTermEntry {
    unresolved_term: UnresolvedTerm,
    resolve_progress: ExprTypeResult<LocalTerm>,
}

#[derive(Default)]
pub(crate) struct UnresolvedTermTable {
    unresolved_terms: Vec<UnresolvedTermEntry>,
    first_unresolved_term: usize,
    expectations: Arena<(Expectation, ExpectationState)>,
    first_unresolved_expectation: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Expectation {
    Err(OriginalTypeError),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExpectationState {
    Unresolved,
}

pub(crate) type ExpectationIdx = ArenaIdx<(Expectation, ExpectationState)>;
pub(crate) type OptionExpectationIdx = OptionArenaIdx<(Expectation, ExpectationState)>;

impl std::ops::Index<ExpectationIdx> for UnresolvedTermTable {
    type Output = Expectation;

    fn index(&self, index: ExpectationIdx) -> &Self::Output {
        &self.expectations[index].0
    }
}

impl UnresolvedTermTable {
    pub(crate) fn unresolved_term(
        &self,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> &UnresolvedTerm {
        &self.unresolved_terms[unresolved_term_idx.0].unresolved_term
    }

    pub(crate) fn intern_unresolved_term(
        &mut self,
        unresolved_term: UnresolvedTerm,
    ) -> UnresolvedTermIdx {
        let position = self
            .unresolved_terms
            .iter()
            .position(|entry| entry.unresolved_term == unresolved_term);
        match position {
            Some(idx) => UnresolvedTermIdx(idx),
            None => {
                let idx = self.unresolved_terms.len();
                self.unresolved_terms.push(UnresolvedTermEntry {
                    unresolved_term,
                    resolve_progress: Ok(LocalTerm::Unresolved(UnresolvedTermIdx(idx))),
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
            Some(expectation) => self
                .expectations
                .intern(
                    (expectation, ExpectationState::Unresolved),
                    |(t0, _), (t1, _)| t0 == t1,
                )
                .into(),
            None => Default::default(),
        }
    }

    pub(crate) fn resolve_term(&mut self, unresolved_term_idx: UnresolvedTermIdx) {
        self.resolve_as_much_as_possible();
        todo!()
    }

    fn resolve_as_much_as_possible(&mut self) {
        todo!()
    }
}
