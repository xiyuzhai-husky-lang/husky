use super::*;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermIdx(usize);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UnresolvedTermEntry {
    unresolved_term: UnresolvedTerm,
    resolve_progress: ExprTypeResult<LocalTerm>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct UnresolvedTermTable {
    unresolved_terms: Vec<UnresolvedTermEntry>,
    first_unresolved_term: usize,
    expectation_rules: Arena<ExpectationRule>,
    first_unresolved_expectation: usize,
}

pub(crate) type ExpectationIdx = ArenaIdx<ExpectationRule>;
pub(crate) type OptionExpectationIdx = OptionArenaIdx<ExpectationRule>;

impl std::ops::Index<UnresolvedTermIdx> for UnresolvedTermTable {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.unresolved_terms[index.0]
    }
}

impl std::ops::Index<ExpectationIdx> for UnresolvedTermTable {
    type Output = ExpectationRule;

    fn index(&self, index: ExpectationIdx) -> &Self::Output {
        &self.expectation_rules[index]
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

    pub(crate) fn add_expectation_rule(
        &mut self,
        ty: LocalTerm,
        expectation: Expectation,
    ) -> OptionExpectationIdx {
        let item = match expectation {
            Expectation::None => return Default::default(),
            Expectation::Type => todo!(),
            Expectation::UnitOrNever => todo!(),
            Expectation::Condition => todo!(),
        };
        self.expectation_rules.alloc_one(item).into()
    }

    pub(crate) fn resolve_term(&mut self, unresolved_term_idx: UnresolvedTermIdx) -> Option<Term> {
        self.resolve_as_much_as_possible();
        todo!()
    }

    fn resolve_as_much_as_possible(&mut self) {
        // ad hoc
        // todo!()
    }

    pub(crate) fn finalize(&mut self) {
        self.resolve_as_much_as_possible();
        // ad hoc
        // todo!()
    }
}
