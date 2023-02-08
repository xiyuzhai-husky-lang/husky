use super::*;
use husky_print_utils::p;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermIdx(usize);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UnresolvedTermEntry {
    unresolved_term: UnresolvedTerm,
    unresolved_term_pattern: UnresolvedTermPattern,
    resolve_progress: ExprTypeResult<LocalTerm>,
}

impl UnresolvedTermEntry {
    pub(crate) fn pattern(&self) -> &UnresolvedTermPattern {
        &self.unresolved_term_pattern
    }

    pub(crate) fn unresolved_term(&self) -> &UnresolvedTerm {
        &self.unresolved_term
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UnresolvedTermPattern {
    ImplicitSymbol,
    Injection {
        function: EntityPath,
        arguments: Vec<LocalTerm>,
    },
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct UnresolvedTermTable {
    implicit_symbol_registry: ImplicitSymbolRegistry,
    unresolved_terms: Vec<UnresolvedTermEntry>,
    first_unresolved_term: usize,
    expectation_rules: Arena<LocalTermExpectationRule>,
    first_unresolved_expectation: usize,
}

pub(crate) type LocalTermExpectationRuleIdx = ArenaIdx<LocalTermExpectationRule>;
pub(crate) type OptionLocalTermExpectationRuleIdx = OptionArenaIdx<LocalTermExpectationRule>;

impl std::ops::Index<UnresolvedTermIdx> for UnresolvedTermTable {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.unresolved_terms[index.0]
    }
}

impl std::ops::Index<LocalTermExpectationRuleIdx> for UnresolvedTermTable {
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
    pub(crate) fn unresolved_term(
        &self,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> &UnresolvedTerm {
        &self.unresolved_term_table().unresolved_terms[unresolved_term_idx.0].unresolved_term
    }

    pub(crate) fn intern_unresolved_term(
        &mut self,
        unresolved_term: UnresolvedTerm,
    ) -> UnresolvedTermIdx {
        let position = self
            .unresolved_term_table()
            .unresolved_terms
            .iter()
            .position(|entry| entry.unresolved_term == unresolved_term);
        match position {
            Some(idx) => UnresolvedTermIdx(idx),
            None => self.alloc_unresolved_term(unresolved_term),
        }
    }

    fn alloc_unresolved_term(&mut self, unresolved_term: UnresolvedTerm) -> UnresolvedTermIdx {
        let idx = self.unresolved_term_table().unresolved_terms.len();
        let unresolved_term_pattern = self.generate_unresolved_term_pattern(&unresolved_term);
        self.unresolved_term_table_mut()
            .unresolved_terms
            .push(UnresolvedTermEntry {
                unresolved_term_pattern,
                unresolved_term,
                resolve_progress: Ok(LocalTerm::Unresolved(UnresolvedTermIdx(idx))),
            });
        UnresolvedTermIdx(idx)
    }

    pub(crate) fn add_expectation_rule(
        &mut self,
        src_expr_idx: ExprIdx,
        target: LocalTerm,
        expectation: LocalTermExpectation,
    ) -> OptionLocalTermExpectationRuleIdx {
        let variant = match expectation {
            LocalTermExpectation::None => return Default::default(),
            LocalTermExpectation::Type => ExpectationRuleVariant::Type,
            LocalTermExpectation::AsBool => ExpectationRuleVariant::AsBool,
            LocalTermExpectation::Return { ty } => todo!(),
            LocalTermExpectation::ImplicitlyConvertibleTo { term } => {
                ExpectationRuleVariant::ImplicitlyConvertibleTo { dst: term }
            }
        };
        let rule = self.new_expectation_rule(src_expr_idx, target, variant);
        self.unresolved_term_table_mut()
            .expectation_rules
            .alloc_one(rule)
            .into()
    }

    pub(crate) fn resolve_term(&mut self, unresolved_term_idx: UnresolvedTermIdx) -> Option<Term> {
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak);
        match self.unresolved_term_table()[unresolved_term_idx].resolve_progress {
            Ok(LocalTerm::Resolved(term)) => Some(term.term()),
            Ok(LocalTerm::Unresolved(_)) => {
                self.unresolved_term_table_mut().unresolved_terms[unresolved_term_idx.0]
                    .resolve_progress = Err(OriginalExprTypeError::UnresolvedTerm.into());
                None
            }
            Err(_) => todo!(),
        }
    }

    fn resolve_as_much_as_possible(&mut self, level: LocalTermResolveLevel) {
        while let Some((idx, action)) = self.next_term_resolve_action(level) {
            let resolve_progress = match action {
                TermResolveAction::SubstituteExpecteeImplicitSymbol {
                    implicit_symbol: expectee_implicit_symbol,
                    substitution,
                } => LocalTermResolveProgress::Resolved {
                    implicit_conversion: LocalTermImplicitConversion::None,
                    local_term: substitution,
                },
            };
            self.unresolved_term_table_mut()
                .expectation_rules
                .update(idx, |rule| rule.set_resolve_progress(resolve_progress))
        }
        // ad hoc
        // todo!()
    }

    pub(crate) fn finalize_unresolved_term_table(&mut self) {
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Strong);
        // ad hoc
        // todo!()
    }

    pub(crate) fn new_implicit_symbol(
        &mut self,
        expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> LocalTerm {
        let new_implicit_symbol = self
            .unresolved_term_table_mut()
            .implicit_symbol_registry
            .new_implicit_symbol(expr_idx, variant);
        self.alloc_unresolved_term(UnresolvedTerm::ImplicitSymbol(new_implicit_symbol))
            .into()
    }

    fn next_term_resolve_action(
        &self,
        level: LocalTermResolveLevel,
    ) -> Option<(LocalTermExpectationRuleIdx, TermResolveAction)> {
        for (idx, rule) in self.unresolved_expectation_rule_iter() {
            if let Some(action) = self.expectation_rule_action(rule, level) {
                return Some((idx, action));
            }
        }
        None
    }

    fn unresolved_expectation_rule_iter(
        &self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &LocalTermExpectationRule)> {
        let table = self.unresolved_term_table();
        table
            .expectation_rules
            .indexed_iter_with_start(table.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermResolveProgress::Unresolved => true,
                LocalTermResolveProgress::Resolved { .. } | LocalTermResolveProgress::Err(_) => {
                    false
                }
            })
    }

    fn generate_unresolved_term_pattern(
        &self,
        unresolved_term: &UnresolvedTerm,
    ) -> UnresolvedTermPattern {
        match unresolved_term {
            UnresolvedTerm::ImplicitSymbol(_) => UnresolvedTermPattern::ImplicitSymbol,
            UnresolvedTerm::Curry {
                parameter_ty,
                return_ty,
            } => todo!(),
            UnresolvedTerm::Application { function, argument } => match function {
                LocalTerm::Resolved(term) => match term.term() {
                    Term::Literal(_) => todo!(),
                    Term::Symbol(_) => todo!(),
                    Term::Entity(entity) => match entity.entity_kind(self.db()) {
                        EntityKind::Module => todo!(),
                        EntityKind::ModuleItem {
                            module_item_kind,
                            connection,
                        } => match module_item_kind {
                            ModuleItemKind::Type(_) => UnresolvedTermPattern::Injection {
                                function: entity,
                                arguments: vec![*argument],
                            },
                            ModuleItemKind::Form(_) => todo!(),
                            ModuleItemKind::Trait => todo!(),
                        },
                        EntityKind::AssociatedItem {
                            associated_item_kind,
                        } => todo!(),
                        EntityKind::Variant => todo!(),
                    },
                    Term::Category(_) => todo!(),
                    Term::Universe(_) => todo!(),
                    Term::Curry(_) => todo!(),
                    Term::Durant(_) => todo!(),
                    Term::Abstraction(_) => todo!(),
                    Term::Application(_) => todo!(),
                    Term::Subentity(_) => todo!(),
                    Term::AsTraitSubentity(_) => todo!(),
                    Term::TraitConstraint(_) => todo!(),
                },
                LocalTerm::Unresolved(_) => todo!(),
            },
            UnresolvedTerm::Abstraction { parameter, body } => todo!(),
            UnresolvedTerm::Durant {
                durant_kind,
                parameter_book_tys,
                return_ty,
            } => todo!(),
            UnresolvedTerm::Subentity {} => todo!(),
            UnresolvedTerm::AsTraitSubentity {} => todo!(),
            UnresolvedTerm::TraitConstraint {} => todo!(),
        }
    }
}

impl<'a> ExprTypeEngine<'a> {}

pub(super) enum TermResolveAction {
    SubstituteExpecteeImplicitSymbol {
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    },
}
