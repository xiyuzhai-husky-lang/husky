use super::*;
use husky_print_utils::p;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};
use vec_like::VecSet;

pub(crate) type LocalTermExpectationIdx = ArenaIdx<LocalTermExpectationEntry>;
pub(crate) type OptionLocalTermExpectationIdx = OptionArenaIdx<LocalTermExpectationEntry>;

impl std::ops::Index<UnresolvedTermIdx> for LocalTermTable {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.unresolved_terms[index]
    }
}

impl std::ops::Index<LocalTermExpectationIdx> for LocalTermTable {
    type Output = LocalTermExpectationEntry;

    fn index(&self, index: LocalTermExpectationIdx) -> &Self::Output {
        &self.expectations[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalTermResolveLevel {
    Weak,
    Strong,
}

impl<'a> ExprTypeEngine<'a> {
    fn next_expectation_effect(
        &self,
        level: LocalTermResolveLevel,
    ) -> Option<(LocalTermExpectationIdx, LocalTermExpectationResolvedOkM)> {
        for (idx, rule) in self.local_term_table().expectations.unresolved_rule_iter() {
            if let Some(action) = self.resolve_expectation(rule, level) {
                return Some((idx, action));
            }
        }
        None
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn resolve_term(
        &mut self,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> Option<ReducedTerm> {
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak);
        self.local_term_table_mut()
            .unresolved_terms
            .resolve_term(unresolved_term_idx)
    }

    pub(crate) fn resolve_as_much_as_possible(&mut self, level: LocalTermResolveLevel) {
        while let Some((rule_idx, effect)) = self.next_expectation_effect(level) {
            if let Some(actions) = self
                .local_term_table_mut()
                .expectations
                .take_effect(rule_idx, effect)
            {
                for action in actions {
                    match action {
                        TermResolveAction::SubstituteImplicitSymbol {
                            implicit_symbol,
                            substitution,
                        } => self.substitute_implicit_symbol(implicit_symbol, substitution),
                    }
                }
            }
        }
    }

    pub(crate) fn finalize_unresolved_term_table(&mut self) {
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Strong);
        // ad hoc
        // todo!()
    }
}

pub(super) enum TermResolveAction {
    SubstituteImplicitSymbol {
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    },
}
