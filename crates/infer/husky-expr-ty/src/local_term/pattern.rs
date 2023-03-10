use thiserror::Error;

use super::*;

pub enum LocalTermPattern {
    Literal(TermLiteral),
    TypeOntology {
        path: Either<CustomTypePath, PreludeTypePath>,
        arguments: SmallVec<[LocalTerm; 2]>,
    },
    Curry {},
    ImplicitSymbol(ImplicitSymbolKind, UnresolvedTermIdx),
    Category(TermCategory),
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
                LocalTermResolveProgress::Unresolved => {
                    LocalTermPattern::from_unresolved(db, term, unresolved_terms)
                }
                LocalTermResolveProgress::PartiallyResolved(term) => {
                    LocalTermPattern::from_unresolved(db, *term, unresolved_terms)
                }
                LocalTermResolveProgress::FullyResolved(term) => {
                    LocalTermPattern::from_resolved(db, *term)
                }
                LocalTermResolveProgress::Err(_) => todo!(),
            },
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LocalTermPatternError {
    #[error("{0}")]
    EntityPathError(#[from] EntityPathError),
}

pub type LocalTermPatternResult<T> = Result<T, LocalTermPatternError>;

impl LocalTermPattern {
    fn from_resolved(db: &dyn ExprTypeDb, term: Term) -> Self {
        match term {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => LocalTermPattern::TypeOntology {
                    path: path.refine(db).expect("should be checked"),
                    arguments: smallvec![],
                },
                TermEntityPath::TypeConstructor(path) => todo!(),
            },
            Term::Category(term) => LocalTermPattern::Category(term),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(term_application) => {
                let expansion = db.term_application_expansion(term);
                match expansion.f() {
                    Term::Symbol(_) => todo!(),
                    Term::EntityPath(path) => match path {
                        TermEntityPath::Form(_) => todo!(),
                        TermEntityPath::Trait(_) => todo!(),
                        TermEntityPath::TypeOntology(path) => LocalTermPattern::TypeOntology {
                            path: path.refine(db).expect("should work"),
                            arguments: expansion
                                .arguments(db)
                                .iter()
                                .copied()
                                .map(Into::into)
                                .collect(),
                        },
                        TermEntityPath::TypeConstructor(_) => todo!(),
                    },
                    Term::Application(_) => todo!(),
                    Term::Subentity(_) => todo!(),
                    Term::AsTraitSubentity(_) => todo!(),
                    Term::TraitConstraint(_) => todo!(),
                    _ => unreachable!(),
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
            UnresolvedTerm::TypeOntology {
                path: ty_path,
                arguments,
            } => todo!(),
            UnresolvedTerm::Ritchie {
                ritchie_kind,
                parameter_tys,
                return_ty,
            } => todo!(),
        }
    }
}
