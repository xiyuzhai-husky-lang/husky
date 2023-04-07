use thiserror::Error;

use super::*;

pub enum LocalTermPattern {
    Literal(TermLiteral),
    TypeOntology {
        path: TypePath,
        refined_path: Either<CustomTypePath, PreludeTypePath>,
        argument_tys: SmallVec<[LocalTerm; 2]>,
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<LocalTerm>,
        parameter_ty: LocalTerm,
        return_ty: LocalTerm,
    },
    ImplicitSymbol(ImplicitSymbolKind, LocalTermIdx),
    Category(TermCategory),
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_contracted_tys: Vec<LocalTermRitchieParameterContractedType>,
        return_ty: LocalTerm,
    },
}

impl LocalTerm {
    /// intended for downstream crates
    pub fn pattern(self, engine: &impl LocalTermEngine<'_>) -> LocalTermPattern {
        self.pattern_inner(engine.db(), engine.unresolved_terms())
    }

    /// intended this crate
    pub(crate) fn pattern_inner(
        self,
        db: &dyn TermDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermPattern {
        match self {
            LocalTerm::Resolved(term) => LocalTermPattern::from_resolved(db, term),
            LocalTerm::Unresolved(term) => match unresolved_terms[term].resolve_progress() {
                Some(LocalTerm::Unresolved(term)) => {
                    LocalTermPattern::from_unresolved(db, term, unresolved_terms)
                }
                Some(LocalTerm::Resolved(term)) => LocalTermPattern::from_resolved(db, term),
                None => todo!(),
            },
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalTermPatternError {
    #[error("{0}")]
    EntityPathError(#[from] EntityPathError),
}

impl LocalTermPattern {
    fn from_resolved(db: &dyn TermDb, term: Term) -> Self {
        match term {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::Hole(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => LocalTermPattern::TypeOntology {
                    path,
                    refined_path: path.refine(db),
                    argument_tys: smallvec![],
                },
                TermEntityPath::TypeConstructor(path) => todo!(),
            },
            Term::Category(term) => LocalTermPattern::Category(term),
            Term::Universe(_) => todo!(),
            Term::Curry(term) => LocalTermPattern::Curry {
                curry_kind: term.curry_kind(db),
                variance: term.variance(db),
                parameter_variable: term.parameter_variable(db).map(Into::into),
                parameter_ty: term.parameter_ty(db).into(),
                return_ty: term.return_ty(db).into(),
            },
            Term::Ritchie(term) => LocalTermPattern::Ritchie {
                ritchie_kind: term.ritchie_kind(db),
                parameter_contracted_tys: term
                    .parameter_contracted_tys(db)
                    .iter()
                    .map(Into::into)
                    .collect(),
                return_ty: term.return_ty(db).into(),
            },
            Term::Abstraction(_) => todo!(),
            Term::Application(term_application) => {
                let expansion = term.application_expansion(db);
                match expansion.function() {
                    TermFunctionReduced::TypeOntology(path) => LocalTermPattern::TypeOntology {
                        path,
                        refined_path: path.refine(db),
                        argument_tys: expansion
                            .arguments(db)
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                    TermFunctionReduced::Trait(_) => todo!(),
                    TermFunctionReduced::Other(_) => todo!(),
                }
            }
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }

    fn from_unresolved(
        db: &dyn TermDb,
        term: LocalTermIdx,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermPattern {
        match unresolved_terms[term].unresolved_term() {
            LocalTermData::ImplicitSymbol(symbol) => {
                LocalTermPattern::ImplicitSymbol(symbol.kind(), term)
            }
            LocalTermData::TypeOntology(term) => {
                let path = term.path();
                LocalTermPattern::TypeOntology {
                    path,
                    refined_path: path.refine(db),
                    argument_tys: term.arguments().into(),
                }
            }
            LocalTermData::Ritchie(_) => todo!(),
            LocalTermData::PlaceType { .. } => todo!(),
        }
    }
}
