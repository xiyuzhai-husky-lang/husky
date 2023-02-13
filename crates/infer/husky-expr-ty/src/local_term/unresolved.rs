use idx_arena::{Arena, ArenaIdx};
use vec_like::VecSet;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UnresolvedTerm {
    ImplicitSymbol(ImplicitSymbol),
    TypeApplication {
        ty: TypePath,
        arguments: Vec<LocalTerm>,
    },
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
    ImplicitLifetime,
    ExprEvalLifetime,
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
pub(crate) struct UnresolvedTermRitchieParameterBookType {
    ty: LocalTerm,
}

impl UnresolvedTermRitchieParameterBookType {
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
    pub(super) fn resolve_term(
        &mut self,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> Option<ReducedTerm> {
        let unresolved_term_entry = &mut self.arena[unresolved_term_idx.0];
        match unresolved_term_entry.resolve_progress {
            LocalTermResolveProgress::FullyResolved(term) => Some(term),
            LocalTermResolveProgress::Unresolved
            | LocalTermResolveProgress::PartiallyResolved(_) => {
                unresolved_term_entry.resolve_progress = LocalTermResolveProgress::Err(
                    OriginalLocalTermResolveError::UnresolvedTerm.into(),
                );
                None
            }
            LocalTermResolveProgress::Err(_) => None,
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
    implicit_symbol_dependencies: VecSet<UnresolvedTermIdx>,
    resolve_progress: LocalTermResolveProgress,
}

impl UnresolvedTermEntry {
    pub(crate) fn unresolved_term(&self) -> &UnresolvedTerm {
        &self.unresolved_term
    }
}

impl UnresolvedTerms {
    fn extract_implicit_symbol_dependencies(
        &self,
        unresolved_term: &UnresolvedTerm,
    ) -> VecSet<UnresolvedTermIdx> {
        let mut dependencies: VecSet<UnresolvedTermIdx> = Default::default();
        match unresolved_term {
            UnresolvedTerm::ImplicitSymbol(_) => (),
            UnresolvedTerm::TypeApplication { ty, arguments } => {
                for argument in arguments {
                    self.extract_local_term_implicit_symbol_dependencies(
                        *argument,
                        &mut dependencies,
                    );
                }
            }
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
        expectation_rule_variant: &LocalTermExpectation,
    ) -> Result<Option<LocalTermExpectation>, &LocalTermResolveError> {
        match expectation_rule_variant {
            LocalTermExpectation::ImplicitlyConversion { destination: dst } => {
                match self.try_substitute_local_term(*dst)? {
                    Some(dst) => Ok(Some(LocalTermExpectation::ImplicitlyConversion {
                        destination: dst,
                    })),
                    None => Ok(None),
                }
            }
            LocalTermExpectation::EqsSort { .. } => Ok(None),
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::EqsRefMutApplication { lifetime } => todo!(),
            LocalTermExpectation::EqsRitchieCallTy => todo!(),
            LocalTermExpectation::InsSort { .. } => todo!(),
            LocalTermExpectation::EqsExactly { destination } => todo!(),
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
        let implicit_symbol_dependencies = self
            .local_term_table()
            .unresolved_terms
            .extract_implicit_symbol_dependencies(&unresolved_term);
        self.local_term_table_mut()
            .unresolved_terms
            .arena
            .push(UnresolvedTermEntry {
                unresolved_term,
                implicit_symbol_dependencies,
                resolve_progress: LocalTermResolveProgress::Unresolved,
            });
        UnresolvedTermIdx(idx)
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
        let new_expectation_rules: Vec<LocalTermExpectationEntry> = Default::default();
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
                    rule.set_resolved(LocalTermExpectationResult::Err(
                        DerivedLocalTermExpectationError::TargetSubstitutionFailure.into(),
                    ));
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
