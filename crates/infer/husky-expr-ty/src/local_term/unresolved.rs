use idx_arena::{Arena, ArenaIdx};
use vec_like::VecSet;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UnresolvedTerm {
    ImplicitSymbol(ImplicitSymbol),
    Curry {
        // todo: include parameter
        // so that the following dependent type is possible:
        // (u: Universe) -> (a: Type u + 1) -> List a -> a
        parameter_ty: LocalTerm,
        return_ty: LocalTerm,
    },
    Application {
        function: LocalTerm,
        argument: LocalTerm,
    },
    Abstraction {
        parameter: TermSymbol,
        body: LocalTerm,
    },
    Durant {
        durant_kind: TermDurantKind,
        parameter_book_tys: Vec<UnresolvedTermDurantParameterBookType>,
        return_ty: LocalTerm,
    },
    Subentity {},
    AsTraitSubentity {},
    TraitConstraint {},
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ImplicitSymbol {
    idx: ImplicitSymbolIdx,
    src_expr_idx: ExprIdx,
    variant: ImplicitSymbolVariant,
}

impl ImplicitSymbol {
    pub(crate) fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub(crate) fn variant(&self) -> &ImplicitSymbolVariant {
        &self.variant
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImplicitSymbolVariant {
    Lifetime,
    UnspecifiedIntegerType,
    UnspecifiedFloatType,
    ImplicitType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ImplicitSymbolIdx(usize);

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ImplicitSymbolRegistry {
    next: usize,
}

impl ImplicitSymbolRegistry {
    fn next(&mut self) -> ImplicitSymbolIdx {
        let idx = ImplicitSymbolIdx(self.next);
        self.next += 1;
        idx
    }

    pub(super) fn new_implicit_symbol(
        &mut self,
        src_expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> ImplicitSymbol {
        ImplicitSymbol {
            idx: self.next(),
            src_expr_idx,
            variant,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermDurantParameterBookType {
    ty: LocalTerm,
}

impl UnresolvedTermDurantParameterBookType {
    pub(crate) fn ty(&self) -> LocalTerm {
        self.ty
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct UnresolvedTerms {
    arena: Vec<UnresolvedTermEntry>,
    first_unresolved_term: usize,
}

impl UnresolvedTerms {
    pub(super) fn resolve_term(&mut self, unresolved_term_idx: UnresolvedTermIdx) -> Option<Term> {
        let unresolved_term_entry = &mut self.arena[unresolved_term_idx.0];
        match unresolved_term_entry.resolve_progress {
            LocalTermResolveProgress::FullyResolved(term) => Some(term.term()),
            LocalTermResolveProgress::Unresolved
            | LocalTermResolveProgress::PartiallyResolved(_) => {
                unresolved_term_entry.resolve_progress = LocalTermResolveProgress::Err(
                    OriginalLocalTermResolveError::UnresolvedTerm.into(),
                );
                None
            }
            LocalTermResolveProgress::Err(_) => todo!(),
        }
    }

    pub(super) fn iter_mut(&mut self) -> impl Iterator<Item = &mut UnresolvedTermEntry> {
        self.arena[self.first_unresolved_term..]
            .iter_mut()
            .filter(|entry| !entry.resolve_progress.is_done())
    }
}

impl LocalTermResolveProgress {
    fn is_done(&self) -> bool {
        match self {
            LocalTermResolveProgress::Unresolved
            | LocalTermResolveProgress::PartiallyResolved(_) => false,
            LocalTermResolveProgress::FullyResolved(_) | LocalTermResolveProgress::Err(_) => true,
        }
    }
}

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

impl UnresolvedTerms {
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

    fn try_substitute_local_term(
        &self,
        local_term: LocalTerm,
    ) -> Result<Option<LocalTerm>, &LocalTermResolveError> {
        match local_term {
            LocalTerm::Resolved(_) => Ok(None),
            LocalTerm::Unresolved(unresolved_term) => {
                match self[unresolved_term].resolve_progress {
                    LocalTermResolveProgress::Unresolved => Ok(None),
                    LocalTermResolveProgress::PartiallyResolved(term) => Ok(Some(term.into())),
                    LocalTermResolveProgress::FullyResolved(term) => Ok(Some(term.into())),
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
                    Some(dst) => Ok(Some(
                        LocalTermExpectationRuleVariant::ImplicitlyConvertibleTo { dst },
                    )),
                    None => Ok(None),
                }
            }
            LocalTermExpectationRuleVariant::Type => Ok(None),
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn intern_unresolved_term(
        &mut self,
        unresolved_term: UnresolvedTerm,
    ) -> UnresolvedTermIdx {
        let position = self
            .local_term_table()
            .unresolved_terms
            .arena
            .iter()
            .position(|entry| entry.unresolved_term == unresolved_term);
        match position {
            Some(idx) => UnresolvedTermIdx(idx),
            None => self.alloc_unresolved_term(unresolved_term),
        }
    }

    fn alloc_unresolved_term(&mut self, unresolved_term: UnresolvedTerm) -> UnresolvedTermIdx {
        let idx = self.local_term_table().unresolved_terms.arena.len();
        let unresolved_term_pattern = self.generate_unresolved_term_pattern(&unresolved_term);
        let implicit_symbol_dependencies = self
            .local_term_table()
            .unresolved_terms
            .extract_implicit_symbol_dependencies(&unresolved_term);
        self.local_term_table_mut()
            .unresolved_terms
            .arena
            .push(UnresolvedTermEntry {
                unresolved_term_pattern,
                unresolved_term,
                implicit_symbol_dependencies,
                resolve_progress: LocalTermResolveProgress::Unresolved,
            });
        UnresolvedTermIdx(idx)
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

    pub(super) fn substitute_implicit_symbol(
        &mut self,
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    ) {
        let substitution_implicit_symbol_dependencies = self
            .local_term_table()
            .unresolved_terms
            .extract_local_term_implicit_symbol_dependencies_standalone(substitution);
        if substitution_implicit_symbol_dependencies.has(implicit_symbol) {
            todo!("report error of cyclic substitution")
        }
        let table = &mut self.local_term_table_mut();
        for entry in table.unresolved_terms.iter_mut() {
            if entry.implicit_symbol_dependencies.has(implicit_symbol) {
                match entry.resolve_progress {
                    LocalTermResolveProgress::Unresolved => todo!(),
                    LocalTermResolveProgress::PartiallyResolved(_) => todo!(),
                    LocalTermResolveProgress::FullyResolved(_)
                    | LocalTermResolveProgress::Err(_) => unreachable!(),
                }
            }
        }
        let new_expectation_rules: Vec<LocalTermExpectationRule> = Default::default();
        for (idx, rule) in table
            .expectation_rules
            .unresolved_expectation_rule_iter_mut()
        {
            let target_substitution = match table
                .unresolved_terms
                .try_substitute_local_term(rule.target())
            {
                Ok(target_substitution) => target_substitution,
                Err(_) => {
                    rule.err(
                        DerivedLocalTermExpectationResolveError::TargetSubstitutionFailure.into(),
                    );
                    continue;
                }
            };
            let variant_substitution = table
                .unresolved_terms
                .try_substitute_expectation_rule_variant(rule.variant());
        }
        let implicit_symbol_entry = &mut table.unresolved_terms.arena[implicit_symbol.0];
        implicit_symbol_entry.resolve_progress = LocalTermResolveProgress::new(substitution);
        implicit_symbol_entry.implicit_symbol_dependencies =
            substitution_implicit_symbol_dependencies;
    }

    pub(crate) fn new_implicit_symbol(
        &mut self,
        expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> LocalTerm {
        let new_implicit_symbol = self
            .local_term_table_mut()
            .implicit_symbol_registry
            .new_implicit_symbol(expr_idx, variant);
        self.alloc_unresolved_term(UnresolvedTerm::ImplicitSymbol(new_implicit_symbol))
            .into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermIdx(usize);

impl std::ops::Index<UnresolvedTermIdx> for UnresolvedTerms {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.arena[index.0]
    }
}
