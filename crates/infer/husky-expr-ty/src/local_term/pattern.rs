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
        parameter_symbol: Option<LocalTerm>,
        parameter_ty: LocalTerm,
        return_ty: LocalTerm,
    },
    ImplicitSymbol(ImplicitSymbolKind, UnresolvedTermIdx),
    Category(TermCategory),
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_liasoned_tys: Vec<LocalTermRitchieParameterLiasonedType>,
        return_ty: LocalTerm,
    },
}

impl LocalTerm {
    pub(crate) fn pattern(
        self,
        db: &dyn ExprTypeDb,
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
    fn from_resolved(db: &dyn ExprTypeDb, term: Term) -> Self {
        match term {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => LocalTermPattern::TypeOntology {
                    path,
                    refined_path: path.refine(db).expect("should be checked"),
                    argument_tys: smallvec![],
                },
                TermEntityPath::TypeConstructor(path) => todo!(),
            },
            Term::Category(term) => LocalTermPattern::Category(term),
            Term::Universe(_) => todo!(),
            Term::Curry(term) => LocalTermPattern::Curry {
                curry_kind: term.curry_kind(db),
                variance: term.variance(db),
                parameter_symbol: term.parameter_symbol(db).map(Into::into),
                parameter_ty: term.parameter_ty(db).into(),
                return_ty: term.return_ty(db).into(),
            },
            Term::Ritchie(term) => LocalTermPattern::Ritchie {
                ritchie_kind: term.ritchie_kind(db),
                parameter_liasoned_tys: term
                    .parameter_liasoned_tys(db)
                    .iter()
                    .map(Into::into)
                    .collect(),
                return_ty: term.return_ty(db).into(),
            },
            Term::Abstraction(_) => todo!(),
            Term::Application(term_application) => {
                let expansion = db.term_application_expansion(term);
                match expansion.function() {
                    TermFunctionReduced::TypeOntology(path) => LocalTermPattern::TypeOntology {
                        path,
                        refined_path: path.refine(db).expect("should work"),
                        argument_tys: expansion
                            .arguments(db)
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect(),
                    },
                }
            }
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }

    fn from_unresolved(
        db: &dyn ExprTypeDb,
        term: UnresolvedTermIdx,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermPattern {
        match unresolved_terms[term].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(symbol) => {
                LocalTermPattern::ImplicitSymbol(symbol.kind(), term)
            }
            UnresolvedTerm::TypeOntology { path, arguments } => LocalTermPattern::TypeOntology {
                path: *path,
                refined_path: path.refine(db).expect("should checked before"),
                argument_tys: arguments.clone(),
            },
            UnresolvedTerm::Ritchie {
                ritchie_kind,
                parameter_tys,
                return_ty,
            } => todo!(),
        }
    }
}
