mod substitution;

pub(crate) use substitution::*;

use idx_arena::{Arena, ArenaIdx};
use vec_like::VecSet;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum UnresolvedTerm {
    ImplicitSymbol(ImplicitSymbol),
    TypeOntology {
        path: TypePath,
        arguments: SmallVec<[LocalTerm; 2]>,
    },
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_tys: Vec<LocalTermRitchieParameter>,
        return_ty: LocalTerm,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct LocalTermRitchieParameter {
    pub(crate) ty: LocalTerm,
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

    pub(crate) fn kind(&self) -> ImplicitSymbolKind {
        self.variant.kind()
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

impl ImplicitSymbolVariant {
    fn kind(&self) -> ImplicitSymbolKind {
        match self {
            ImplicitSymbolVariant::ImplicitLifetime => ImplicitSymbolKind::ImplicitLifetime,
            ImplicitSymbolVariant::ExprEvalLifetime => ImplicitSymbolKind::ExprEvalLifetime,
            ImplicitSymbolVariant::UnspecifiedIntegerType => {
                ImplicitSymbolKind::UnspecifiedIntegerType
            }
            ImplicitSymbolVariant::UnspecifiedFloatType => ImplicitSymbolKind::UnspecifiedFloatType,
            ImplicitSymbolVariant::ImplicitType => ImplicitSymbolKind::ImplicitType,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitSymbolKind {
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

    fn new_implicit_symbol(
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
pub struct UnresolvedTerms {
    implicit_symbol_registry: ImplicitSymbolRegistry,
    data: Vec<UnresolvedTermEntry>,
    first_unresolved_term: usize,
}

impl UnresolvedTerms {
    pub(super) fn force_resolve_term(
        &mut self,
        unresolved_term_idx: UnresolvedTermIdx,
    ) -> Option<Term> {
        let unresolved_term_entry = &mut self.data[unresolved_term_idx.0];
        match unresolved_term_entry.resolve_progress {
            Ok(LocalTerm::Resolved(term)) => Some(term),
            Ok(LocalTerm::Unresolved(_)) => {
                unresolved_term_entry.resolve_progress =
                    Err(OriginalLocalTermResolveError::UnresolvedTerm.into());
                None
            }
            Err(_) => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &UnresolvedTermEntry> {
        self.data[self.first_unresolved_term..].iter()
    }

    pub(super) fn unresolved_iter_mut(&mut self) -> impl Iterator<Item = &mut UnresolvedTermEntry> {
        self.data[self.first_unresolved_term..]
            .iter_mut()
            .filter(|entry| !entry.is_done())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnresolvedTermEntry {
    src_expr_idx: ExprIdx,
    unresolved_term: UnresolvedTerm,
    implicit_symbol_dependencies: VecSet<UnresolvedTermIdx>,
    resolve_progress: LocalTermResolveResult<LocalTerm>,
}

impl UnresolvedTermEntry {
    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub(crate) fn unresolved_term(&self) -> &UnresolvedTerm {
        &self.unresolved_term
    }

    pub fn original_error(&self) -> Option<&OriginalLocalTermResolveError> {
        match self.resolve_progress {
            Err(LocalTermResolveError::Original(ref e)) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn resolve_progress(&self) -> Option<LocalTerm> {
        match self.resolve_progress {
            Ok(resolve_progress) => Some(resolve_progress),
            Err(_) => None,
        }
    }

    pub(crate) fn is_done(&self) -> bool {
        match self.resolve_progress {
            Ok(LocalTerm::Resolved(_)) | Err(_) => true,
            Ok(LocalTerm::Unresolved(_)) => false,
        }
    }
}

impl UnresolvedTerms {
    pub(super) fn new_implicit_symbol(
        &mut self,
        src_expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> UnresolvedTermIdx {
        let new_implicit_symbol = self
            .implicit_symbol_registry
            .new_implicit_symbol(src_expr_idx, variant);
        self.alloc_unresolved_term(
            src_expr_idx,
            UnresolvedTerm::ImplicitSymbol(new_implicit_symbol),
        )
    }

    pub(crate) fn new_implicit_symbol_from_parameter_symbol(
        &mut self,
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        parameter_symbol: TermSymbol,
    ) -> UnresolvedTermIdx {
        let variant = match parameter_symbol.ty(db) {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(_) => todo!(),
            Term::Category(cat) if cat.universe().raw() == 0 => todo!(),
            Term::Category(cat) if cat.universe().raw() == 1 => ImplicitSymbolVariant::ImplicitType,
            Term::Category(_) => todo!(),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        };
        self.new_implicit_symbol(src_expr_idx, variant)
    }

    pub(super) fn intern_unresolved_term(
        &mut self,
        src_expr_idx: ExprIdx,
        unresolved_term: UnresolvedTerm,
    ) -> LocalTerm {
        // todo: check that unresolved_term is indeed unresolved;
        // if not, return reduced term
        let position = self
            .data
            .iter()
            .position(|entry| entry.unresolved_term == unresolved_term);
        match position {
            Some(idx) => UnresolvedTermIdx(idx),
            None => self.alloc_unresolved_term(src_expr_idx, unresolved_term),
        }
        .into()
    }

    fn alloc_unresolved_term(
        &mut self,
        src_expr_idx: ExprIdx,
        unresolved_term: UnresolvedTerm,
    ) -> UnresolvedTermIdx {
        let idx = UnresolvedTermIdx(self.data.len());
        let implicit_symbol_dependencies =
            self.extract_implicit_symbol_dependencies(&unresolved_term);
        self.data.push(UnresolvedTermEntry {
            src_expr_idx,
            unresolved_term,
            implicit_symbol_dependencies,
            resolve_progress: Ok(LocalTerm::Unresolved(idx)),
        });
        idx
    }

    fn extract_implicit_symbol_dependencies(
        &self,
        unresolved_term: &UnresolvedTerm,
    ) -> VecSet<UnresolvedTermIdx> {
        let mut dependencies: VecSet<UnresolvedTermIdx> = Default::default();
        let mut f = |term: LocalTerm| {
            self.extract_local_term_implicit_symbol_dependencies(term, &mut dependencies)
        };
        match unresolved_term {
            UnresolvedTerm::ImplicitSymbol(_) => (),
            UnresolvedTerm::TypeOntology {
                path: ty,
                arguments,
            } => arguments.iter().copied().for_each(f),
            UnresolvedTerm::Ritchie {
                ritchie_kind,
                parameter_tys,
                return_ty,
            } => {
                f(*return_ty);
                parameter_tys
                    .iter()
                    .map(|parameter_ty| parameter_ty.ty)
                    .for_each(f);
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

    pub(super) fn try_reduce_local_term(
        &self,
        local_term: LocalTerm,
    ) -> Result<Option<LocalTerm>, &LocalTermResolveError> {
        match local_term {
            LocalTerm::Resolved(_) => Ok(None),
            LocalTerm::Unresolved(unresolved_term) => {
                match self[unresolved_term].resolve_progress {
                    Ok(resolve_progress) if resolve_progress == local_term => Ok(None),
                    Ok(resolve_progress) => Ok(Some(resolve_progress)),
                    Err(ref e) => Err(e),
                }
            }
        }
    }

    fn try_substitute_expectation_rule_variant<'a>(
        &'a self,
        expectation_rule_variant: &LocalTermExpectation,
    ) -> Result<Option<LocalTermExpectation>, &'a LocalTermResolveError> {
        match expectation_rule_variant {
            LocalTermExpectation::ImplicitlyConvertible(expectation) => {
                expectation.try_substitute_unresolved_local_term(&self)
            }
            LocalTermExpectation::ExplicitlyConvertible(expectation) => {
                expectation.try_substitute_unresolved_local_term(&self)
            }
            LocalTermExpectation::EqsSort(_) => Ok(None),
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::EqsRefMutApplication(_) => todo!(),
            LocalTermExpectation::EqsFunctionType(_) => todo!(),
            LocalTermExpectation::InsSort(_) => Ok(None),
            LocalTermExpectation::EqsExactly(_) => todo!(),
            LocalTermExpectation::AnyOriginal(_) => Ok(None),
            LocalTermExpectation::AnyDerived(_) => Ok(None),
        }
    }
}

impl LocalTermRegion {
    pub(super) fn substitute_implicit_symbol(
        &mut self,
        db: &dyn ExprTypeDb,
        implicit_symbol: UnresolvedTermIdx,
        substitution: LocalTerm,
    ) {
        let substitution_implicit_symbol_dependencies = self
            .unresolved_terms
            .extract_local_term_implicit_symbol_dependencies_standalone(substitution);
        if substitution_implicit_symbol_dependencies.has(implicit_symbol) {
            todo!("report error of cyclic substitution")
        }
        let unresolved_terms = &mut self.unresolved_terms;
        for i in 0..unresolved_terms.data.len() {
            let entry = &unresolved_terms.data[i];
            if !entry.implicit_symbol_dependencies.has(implicit_symbol) {
                continue;
            }
            let resolve_progress = entry.resolve_progress().unwrap();
            let resolve_progress_pattern = resolve_progress.pattern(db, unresolved_terms);
            let resolve_progress = match resolve_progress_pattern {
                LocalTermPattern::Literal(_) => todo!(),
                LocalTermPattern::TypeOntology {
                    path, arguments, ..
                } => {
                    let arguments = arguments
                        .iter()
                        .map(|argument| argument.resolve_progress(unresolved_terms).unwrap())
                        .collect();
                    Ok(LocalTerm::new_ty_ontology_application(
                        db,
                        unresolved_terms,
                        entry.src_expr_idx,
                        path,
                        arguments,
                    ))
                }
                LocalTermPattern::Curry {} => todo!(),
                LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
                LocalTermPattern::Category(_) => todo!(),
            };
            unresolved_terms.data[i].resolve_progress = resolve_progress
        }
        let new_expectation_rules: Vec<LocalTermExpectationRule> = Default::default();
        for (idx, rule) in self.expectations.unresolved_indexed_iter_mut() {
            let target_substitution =
                match self.unresolved_terms.try_reduce_local_term(rule.expectee()) {
                    Ok(target_substitution) => target_substitution,
                    Err(_) => {
                        rule.set_resolved(Err(
                            DerivedLocalTermExpectationError::TargetSubstitutionFailure.into(),
                        ));
                        continue;
                    }
                };
            let variant_substitution = self
                .unresolved_terms
                .try_substitute_expectation_rule_variant(rule.variant());
        }
        let implicit_symbol_entry = &mut self.unresolved_terms.data[implicit_symbol.0];
        implicit_symbol_entry.resolve_progress = Ok(substitution);
        implicit_symbol_entry.implicit_symbol_dependencies =
            substitution_implicit_symbol_dependencies;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnresolvedTermIdx(usize);

impl std::ops::Index<UnresolvedTermIdx> for UnresolvedTerms {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.data[index.0]
    }
}
