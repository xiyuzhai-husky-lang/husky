mod implicit_symbol;
mod place;
mod richie;
mod substitution;
mod ty_ontology;

pub use self::implicit_symbol::*;
pub use self::place::*;
pub use self::richie::*;
pub use self::ty_ontology::*;

pub(crate) use self::substitution::*;

use super::*;
use idx_arena::{Arena, ArenaIdx};
use vec_like::VecSet;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TermDb)]
#[enum_class::from_variants]
pub enum LocalTermData {
    ImplicitSymbol(ImplicitSymbol),
    TypeOntology(LocalTermTypeOntology),
    Ritchie(LocalTermRitchie),
    QualifiedType(LocalTermPlaceType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitSymbol {
    idx: ImplicitSymbolIdx,
    src_expr_idx: ExprIdx,
    variant: ImplicitSymbolVariant,
}

impl ImplicitSymbol {
    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub fn variant(&self) -> &ImplicitSymbolVariant {
        &self.variant
    }

    pub fn kind(&self) -> ImplicitSymbolKind {
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
    unresolved_term: LocalTermData,
    implicit_symbol_dependencies: VecSet<UnresolvedTermIdx>,
    resolve_progress: LocalTermResolveResult<LocalTerm>,
}

impl UnresolvedTermEntry {
    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub fn unresolved_term(&self) -> &LocalTermData {
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
    pub(super) fn intern_unresolved_term(
        &mut self,
        src_expr_idx: ExprIdx,
        unresolved_term: LocalTermData,
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
        unresolved_term: LocalTermData,
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
        unresolved_term: &LocalTermData,
    ) -> VecSet<UnresolvedTermIdx> {
        let mut dependencies: VecSet<UnresolvedTermIdx> = Default::default();
        match unresolved_term {
            LocalTermData::ImplicitSymbol(_) => (), // self is not included
            LocalTermData::TypeOntology(term) => {
                term.extract_implicit_symbol_dependencies(self, &mut dependencies)
            }
            LocalTermData::Ritchie(term) => {
                term.extract_implicit_symbol_dependencies(self, &mut dependencies)
            }
            LocalTermData::QualifiedType(term) => todo!(),
        }
        dependencies
    }

    fn extract_implicit_symbol_dependencies_aux(
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
                    LocalTermData::ImplicitSymbol(_) => dependencies.insert(unresolved_term),
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
        self.extract_implicit_symbol_dependencies_aux(local_term, &mut dependencies);
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
        db: &dyn TermDb,
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
            let resolve_progress = if resolve_progress == implicit_symbol.into() {
                Ok(substitution)
            } else {
                let resolve_progress_pattern = resolve_progress.pattern_inner(db, unresolved_terms);
                match resolve_progress_pattern {
                    LocalTermPattern::Literal(_) => todo!(),
                    LocalTermPattern::TypeOntology {
                        path,
                        argument_tys: arguments,
                        ..
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
                    LocalTermPattern::Curry { .. } => todo!(),
                    LocalTermPattern::Category(_) => todo!(),
                    _ => Ok(resolve_progress),
                }
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

impl UnresolvedTermIdx {
    pub(crate) fn resolve_progress(
        self,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermResolveResultRef<LocalTerm> {
        match unresolved_terms[self].resolve_progress {
            Ok(term) => Ok(term),
            Err(ref e) => Err(e),
        }
    }
}

impl std::ops::Index<UnresolvedTermIdx> for UnresolvedTerms {
    type Output = UnresolvedTermEntry;

    fn index(&self, index: UnresolvedTermIdx) -> &Self::Output {
        &self.data[index.0]
    }
}
