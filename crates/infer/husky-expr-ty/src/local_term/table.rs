use super::*;
use husky_print_utils::p;
use idx_arena::{Arena, ArenaIdx, OptionArenaIdx};
use vec_like::VecSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermIdx(usize);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UnresolvedTermEntry {
    unresolved_term: UnresolvedTerm,
    unresolved_term_pattern: UnresolvedTermPattern,
    implicit_symbol_dependencies: VecSet<UnresolvedTermIdx>,
    resolve_progress: LocalTermResolveProgress,
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

impl UnresolvedTermTable {
    fn extract_implicit_symbol_dependencies(
        &self,
        unresolved_term: &UnresolvedTerm,
    ) -> VecSet<UnresolvedTermIdx> {
        let mut dependencies: VecSet<UnresolvedTermIdx> = Default::default();
        match unresolved_term {
            UnresolvedTerm::ImplicitSymbol(_) => (),
            UnresolvedTerm::Curry {
                parameter_ty,
                return_ty,
            } => {
                self.extract_local_term_implicit_symbol_dependencies(
                    *parameter_ty,
                    &mut dependencies,
                );
                self.extract_local_term_implicit_symbol_dependencies(*return_ty, &mut dependencies);
            }
            UnresolvedTerm::Application { function, argument } => {
                self.extract_local_term_implicit_symbol_dependencies(*function, &mut dependencies);
                self.extract_local_term_implicit_symbol_dependencies(*argument, &mut dependencies);
            }
            UnresolvedTerm::Abstraction { parameter, body } => {
                self.extract_local_term_implicit_symbol_dependencies(*body, &mut dependencies);
            }
            UnresolvedTerm::Durant {
                durant_kind,
                parameter_book_tys,
                return_ty,
            } => {
                for parameter_book_ty in parameter_book_tys {
                    self.extract_local_term_implicit_symbol_dependencies(
                        parameter_book_ty.ty(),
                        &mut dependencies,
                    );
                }
                self.extract_local_term_implicit_symbol_dependencies(*return_ty, &mut dependencies);
            }
            UnresolvedTerm::Subentity {} => todo!(),
            UnresolvedTerm::AsTraitSubentity {} => todo!(),
            UnresolvedTerm::TraitConstraint {} => todo!(),
        }
        dependencies
    }

    fn extract_local_term_implicit_symbol_dependencies(
        &self,
        local_term: impl Into<LocalTerm>,
        dependencies: &mut VecSet<UnresolvedTermIdx>,
    ) {
        let local_term: LocalTerm = local_term.into();
        match local_term {
            LocalTerm::Resolved(_) => (),
            LocalTerm::Unresolved(unresolved_term) => {
                let unresolved_term_entry = &self[unresolved_term];
                match unresolved_term_entry.unresolved_term {
                    UnresolvedTerm::ImplicitSymbol(_) => dependencies.insert(unresolved_term),
                    _ => dependencies.extend(&unresolved_term_entry.implicit_symbol_dependencies),
                }
            }
        }
    }

    fn extract_local_term_implicit_symbol_dependencies_standalone(
        &self,
        local_term: impl Into<LocalTerm>,
    ) -> VecSet<UnresolvedTermIdx> {
        let mut dependencies: VecSet<UnresolvedTermIdx> = Default::default();
        self.extract_local_term_implicit_symbol_dependencies(local_term, &mut dependencies);
        dependencies
    }

    fn unresolved_expectation_rule_iter(
        &self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &LocalTermExpectationRule)> {
        self.expectation_rules
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_)
                | LocalTermExpectationResolveProgress::Err(_) => false,
            })
    }

    fn unresolved_expectation_rule_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (LocalTermExpectationRuleIdx, &mut LocalTermExpectationRule)> {
        self.expectation_rules
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_)
                | LocalTermExpectationResolveProgress::Err(_) => false,
            })
    }

    fn try_substitute_local_term(
        &self,
        local_term: LocalTerm,
    ) -> Result<Option<LocalTerm>, &LocalTermResolveError> {
        match local_term {
            LocalTerm::Resolved(_) => Ok(None),
            LocalTerm::Unresolved(unresolved_term) => {
                match self[unresolved_term].resolve_progress {
                    LocalTermResolveProgress::Ok(term)
                        if term == LocalTerm::Unresolved(unresolved_term) =>
                    {
                        Ok(None)
                    }
                    LocalTermResolveProgress::Ok(term) => Ok(Some(term)),
                    LocalTermResolveProgress::Err(ref e) => Err(e),
                }
            }
        }
    }

    fn try_substitute_expectation_rule_variant(
        &self,
        expectation_rule_variant: &LocalTermExpectationRuleVariant,
    ) -> Result<Option<LocalTermExpectationRuleVariant>, &LocalTermResolveError> {
        match expectation_rule_variant {
            LocalTermExpectationRuleVariant::AsBool => Ok(None),
            LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst } => {
                match self.try_substitute_local_term(*dst)? {
                    Some(_) => todo!(),
                    None => Ok(None),
                }
            }
            LocalTermExpectationRuleVariant::Type => Ok(None),
        }
    }
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
        let implicit_symbol_dependencies = self
            .unresolved_term_table()
            .extract_implicit_symbol_dependencies(&unresolved_term);
        self.unresolved_term_table_mut()
            .unresolved_terms
            .push(UnresolvedTermEntry {
                unresolved_term_pattern,
                unresolved_term,
                implicit_symbol_dependencies,
                resolve_progress: LocalTermResolveProgress::Ok(LocalTerm::Unresolved(
                    UnresolvedTermIdx(idx),
                )),
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
            LocalTermExpectation::Type => LocalTermExpectationRuleVariant::Type,
            LocalTermExpectation::AsBool => LocalTermExpectationRuleVariant::AsBool,
            LocalTermExpectation::Return { ty } => todo!(),
            LocalTermExpectation::ImplicitlyConvertibleTo { term } => {
                LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst: term }
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
            LocalTermResolveProgress::Ok(LocalTerm::Resolved(term)) => Some(term.term()),
            LocalTermResolveProgress::Ok(LocalTerm::Unresolved(_)) => {
                self.unresolved_term_table_mut().unresolved_terms[unresolved_term_idx.0]
                    .resolve_progress = LocalTermResolveProgress::Err(
                    OriginalLocalTermResolveError::UnresolvedTerm.into(),
                );
                None
            }
            LocalTermResolveProgress::Err(_) => todo!(),
        }
    }

    fn resolve_as_much_as_possible(&mut self, level: LocalTermResolveLevel) {
        while let Some((idx, effect)) = self.next_expectation_effect(level) {
            self.unresolved_term_table_mut()
                .expectation_rules
                .update(idx, |rule| rule.resolve(effect.expectation_resolved()));
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

    fn substitute_implicit_symbol(
        &mut self,
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    ) {
        let substitution_implicit_symbol_dependencies = self
            .unresolved_term_table()
            .extract_local_term_implicit_symbol_dependencies_standalone(substitution);
        if substitution_implicit_symbol_dependencies.has(implicit_symbol) {
            todo!("report error of cyclic substitution")
        }
        let table = &mut self.unresolved_term_table_mut();
        for entry in table.unresolved_terms[table.first_unresolved_term..].iter_mut() {
            if entry.implicit_symbol_dependencies.has(implicit_symbol) {
                todo!()
            }
        }
        let new_expectation_rules: Vec<LocalTermExpectationRule> = Default::default();
        for (idx, rule) in table.unresolved_expectation_rule_iter_mut() {
            // let target_substitution = match table.try_substitute_local_term(rule.target()) {
            //     Ok(target_substitution) => target_substitution,
            //     Err(_) => {
            //         rule.err(
            //             DerivedLocalTermExpectationResolveError::TargetSubstitutionFailure.into(),
            //         );
            //         continue;
            //     }
            // };
            // let variant_substitution =
            //     table.try_substitute_expectation_rule_variant(rule.variant());
            todo!()
        }
        let implicit_symbol_entry = &mut table.unresolved_terms[implicit_symbol.0];
        implicit_symbol_entry.resolve_progress = LocalTermResolveProgress::Ok(substitution);
        implicit_symbol_entry.implicit_symbol_dependencies =
            substitution_implicit_symbol_dependencies;
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

    fn next_expectation_effect(
        &self,
        level: LocalTermResolveLevel,
    ) -> Option<(LocalTermExpectationRuleIdx, LocalTermExpectationEffect)> {
        for (idx, rule) in self
            .unresolved_term_table()
            .unresolved_expectation_rule_iter()
        {
            if let Some(action) = self.expectation_rule_effect(rule, level) {
                return Some((idx, action));
            }
        }
        None
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
    SubstituteImplicitSymbol {
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    },
}
