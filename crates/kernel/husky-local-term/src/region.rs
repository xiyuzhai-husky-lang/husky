use super::*;
use husky_print_utils::p;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};
use vec_like::VecSet;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LocalTermRegion {
    pub(super) unresolved_terms: UnresolvedTerms,
    pub(super) expectations: LocalTermExpectations,
}

impl LocalTermRegion {
    pub fn unresolved_terms(&self) -> &UnresolvedTerms {
        &self.unresolved_terms
    }

    pub fn expectations(&self) -> &LocalTermExpectations {
        &self.expectations
    }

    pub fn new_implicit_symbol(
        &mut self,
        src_expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> UnresolvedTermIdx {
        self.unresolved_terms
            .new_implicit_symbol(src_expr_idx, variant)
    }

    pub fn intern_unresolved_term(
        &mut self,
        src_expr_idx: ExprIdx,
        unresolved_term: LocalTermData,
    ) -> LocalTerm {
        self.unresolved_terms
            .intern_unresolved_term(src_expr_idx, unresolved_term)
    }
}

pub type LocalTermExpectationIdx = ArenaIdx<LocalTermExpectationRule>;
pub type OptionLocalTermExpectationIdx = OptionArenaIdx<LocalTermExpectationRule>;

impl std::ops::Index<UnresolvedTermIdx> for LocalTermRegion {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.unresolved_terms[index]
    }
}

impl std::ops::Index<LocalTermExpectationIdx> for LocalTermRegion {
    type Output = LocalTermExpectationRule;

    fn index(&self, index: LocalTermExpectationIdx) -> &Self::Output {
        &self.expectations[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalTermResolveLevel {
    Weak,
    Strong,
}

impl LocalTermRegion {
    fn next_expectation_effect(
        &mut self,
        db: &dyn TermDb,
        level: LocalTermResolveLevel,
    ) -> Option<(LocalTermExpectationIdx, LocalTermExpectationEffect)> {
        for (idx, rule) in self.expectations.unresolved_rule_iter() {
            if let Some(action) = rule.resolve_expectation(db, &mut self.unresolved_terms, level) {
                return Some((idx, action));
            }
        }
        None
    }

    pub fn resolve_term(
        &mut self,
        db: &dyn TermDb,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> Option<Term> {
        self.resolve_as_much_as_possible(db, LocalTermResolveLevel::Weak);
        self.unresolved_terms
            .force_resolve_term(unresolved_term_idx)
    }

    pub fn resolve_as_much_as_possible(&mut self, db: &dyn TermDb, level: LocalTermResolveLevel) {
        while let Some((rule_idx, effect)) = self.next_expectation_effect(db, level) {
            if let Some(actions) = self.expectations.take_effect(rule_idx, effect) {
                for action in actions {
                    match action {
                        TermResolveAction::SubstituteImplicitSymbol {
                            implicit_symbol,
                            substitution,
                        } => self.substitute_implicit_symbol(db, implicit_symbol, substitution),
                        TermResolveAction::AddExpectation {
                            src_expr_idx,
                            expectee,
                            expectation,
                        } => {
                            self.add_expectation_rule(src_expr_idx, expectee, expectation);
                        }
                    }
                }
            }
        }
    }

    pub fn finalize_unresolved_term_table(&mut self, db: &dyn TermDb) {
        self.resolve_as_much_as_possible(db, LocalTermResolveLevel::Strong);
        // ad hoc
        // todo!()
    }
}

pub(super) enum TermResolveAction {
    SubstituteImplicitSymbol {
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    },
    AddExpectation {
        src_expr_idx: ExprIdx,
        expectee: LocalTerm,
        expectation: LocalTermExpectation,
    },
}
