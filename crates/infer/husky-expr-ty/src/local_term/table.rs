use super::*;
use husky_print_utils::p;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};
use vec_like::VecSet;

pub(crate) type LocalTermExpectationRuleIdx = ArenaIdx<LocalTermExpectationRule>;
pub(crate) type OptionLocalTermExpectationRuleIdx = OptionArenaIdx<LocalTermExpectationRule>;

impl std::ops::Index<UnresolvedTermIdx> for LocalTermTable {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.unresolved_terms[index]
    }
}

impl std::ops::Index<LocalTermExpectationRuleIdx> for LocalTermTable {
    type Output = LocalTermExpectationRule;

    fn index(&self, index: LocalTermExpectationRuleIdx) -> &Self::Output {
        &self.expectation_rules[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LocalTermResolveLevel {
    Weak,
    Strong,
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn add_expectation_rule(
        &mut self,
        src_expr_idx: ExprIdx,
        target: LocalTerm,
        expectation: LocalTermExpectation,
    ) -> OptionLocalTermExpectationRuleIdx {
        let variant = match expectation {
            LocalTermExpectation::None => return Default::default(),
            LocalTermExpectation::TypeType => LocalTermExpectationRuleVariant::Type,
            LocalTermExpectation::CastibleAsBool => LocalTermExpectationRuleVariant::AsBool,
            LocalTermExpectation::FrameVariableType => {
                LocalTermExpectationRuleVariant::FrameVariableType
            }
            LocalTermExpectation::Return { ty } => todo!(),
            LocalTermExpectation::ImplicitlyConvertibleTo { ty: term } => {
                LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst: term }
            }
            LocalTermExpectation::RefMut { lifetime } => {
                LocalTermExpectationRuleVariant::RefMut { lifetime }
            }
        };
        let rule = self.new_expectation_rule(src_expr_idx, target, variant);
        self.local_term_table_mut()
            .expectation_rules
            .alloc_rule(rule)
            .into()
    }

    fn next_expectation_effect(
        &self,
        level: LocalTermResolveLevel,
    ) -> Option<(LocalTermExpectationRuleIdx, LocalTermExpectationEffect)> {
        for (idx, rule) in self
            .local_term_table()
            .expectation_rules
            .unresolved_rule_iter()
        {
            if let Some(action) = self.expectation_rule_effect(rule, level) {
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

    fn resolve_as_much_as_possible(&mut self, level: LocalTermResolveLevel) {
        while let Some((rule_idx, effect)) = self.next_expectation_effect(level) {
            self.local_term_table_mut()
                .expectation_rules
                .take_effect(rule_idx, &effect);
            for action in effect.actions() {
                match action {
                    TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol,
                        substitution,
                    } => self.substitute_implicit_symbol(implicit_symbol, substitution),
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
