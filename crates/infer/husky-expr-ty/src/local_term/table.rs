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
    implicit_symbol_registry: ImplicitSymbolRegistry,
    unresolved_terms: Vec<UnresolvedTermEntry>,
    first_unresolved_term: usize,
    expectation_rules: Arena<LocalTermExpectationRule>,
    first_unresolved_expectation: usize,
}

pub(crate) type ExpectationIdx = ArenaIdx<LocalTermExpectationRule>;
pub(crate) type OptionExpectationIdx = OptionArenaIdx<LocalTermExpectationRule>;

impl std::ops::Index<UnresolvedTermIdx> for UnresolvedTermTable {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.unresolved_terms[index.0]
    }
}

impl std::ops::Index<ExpectationIdx> for UnresolvedTermTable {
    type Output = LocalTermExpectationRule;

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
            None => self.alloc_unresolved_term(unresolved_term),
        }
    }

    fn alloc_unresolved_term(&mut self, unresolved_term: UnresolvedTerm) -> UnresolvedTermIdx {
        let idx = self.unresolved_terms.len();
        self.unresolved_terms.push(UnresolvedTermEntry {
            unresolved_term,
            resolve_progress: Ok(LocalTerm::Unresolved(UnresolvedTermIdx(idx))),
        });
        UnresolvedTermIdx(idx)
    }

    pub(crate) fn add_expectation_rule(
        &mut self,
        db: &dyn ExprTypeDb,
        term_menu: &TermMenu,
        target: LocalTerm,
        expectation: LocalTermExpectation,
    ) -> OptionExpectationIdx {
        let variant = match expectation {
            LocalTermExpectation::None => return Default::default(),
            LocalTermExpectation::Type => ExpectationRuleVariant::Type,
            LocalTermExpectation::Condition => ExpectationRuleVariant::AsBool,
            LocalTermExpectation::Return { ty } => todo!(),
            LocalTermExpectation::ImplicitlyConvertibleTo { term } => {
                ExpectationRuleVariant::ImplicitlyConvertibleTo { term }
            }
        };
        let rule = LocalTermExpectationRule::new(db, term_menu, target, variant);
        self.expectation_rules.alloc_one(rule).into()
    }

    pub(crate) fn resolve_term(&mut self, unresolved_term_idx: UnresolvedTermIdx) -> Option<Term> {
        self.resolve_as_much_as_possible();
        match self[unresolved_term_idx].resolve_progress {
            Ok(LocalTerm::Resolved(term)) => Some(term),
            Ok(LocalTerm::Unresolved(_)) => {
                self.unresolved_terms[unresolved_term_idx.0].resolve_progress =
                    Err(OriginalExprTypeError::UnresolvedTerm.into());
                None
            }
            Err(_) => todo!(),
        }
    }

    fn resolve_as_much_as_possible(&mut self) {
        while let Some(action) = self.next_term_resolve_action() {
            todo!()
        }
        // ad hoc
        // todo!()
    }

    pub(crate) fn finalize(&mut self) {
        self.resolve_as_much_as_possible();
        // ad hoc
        // todo!()
    }

    pub(crate) fn new_implicit_symbol(
        &mut self,
        expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> LocalTerm {
        let new_implicit_symbol = self
            .implicit_symbol_registry
            .new_implicit_symbol(expr_idx, variant);
        self.alloc_unresolved_term(UnresolvedTerm::ImplicitSymbol(new_implicit_symbol))
            .into()
    }

    fn next_term_resolve_action(&self) -> Option<TermResolveAction> {
        for expectation_rule in self.unresolved_expectation_rule_iter() {
            todo!()
        }
        None
    }

    fn unresolved_expectation_rule_iter(&self) -> impl Iterator<Item = &LocalTermExpectationRule> {
        self.expectation_rules.data()[self.first_unresolved_expectation..]
            .iter()
            .filter(|rule| match rule.resolve_progress() {
                LocalTermResolveProgress::Unresolved => true,
                LocalTermResolveProgress::Resolved { .. } | LocalTermResolveProgress::Err(_) => {
                    false
                }
            })
    }
}

enum TermResolveAction {}
