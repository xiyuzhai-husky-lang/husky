mod expectation;
mod pattern;
mod progress;
mod region;
mod unresolved;
mod utils;

use husky_print_utils::p;

pub use self::expectation::*;
pub use self::progress::*;

pub(crate) use self::pattern::*;
pub(crate) use self::region::*;
pub(crate) use self::unresolved::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprTypeDb, jar = ExprTypeJar)]
#[enum_class::from_variants]
pub enum LocalTerm {
    Resolved(Term),
    Unresolved(UnresolvedTermIdx),
}

impl LocalTerm {
    pub(crate) fn unresolved(self) -> Option<UnresolvedTermIdx> {
        match self {
            LocalTerm::Resolved(_) => None,
            LocalTerm::Unresolved(idx) => Some(idx),
        }
    }

    pub(crate) fn new_application(
        db: &dyn ExprTypeDb,
        src_expr_idx: ExprIdx,
        function: impl Into<LocalTerm>,
        argument: impl Into<LocalTerm>,
        local_term_region: &mut LocalTermRegion,
    ) -> TermResult<Self> {
        match (function.into(), argument.into()) {
            (LocalTerm::Resolved(function), LocalTerm::Resolved(argument)) => {
                Ok(TermApplication::new(db, function, argument)?.into())
            }
            (LocalTerm::Resolved(function), argument) => {
                let expansion = db.term_application_expansion(function);
                match expansion.f() {
                    Term::Literal(_) => todo!(),
                    Term::Symbol(_) => todo!(),
                    Term::EntityPath(path) => match path {
                        TermEntityPath::Form(_) => todo!(),
                        TermEntityPath::Trait(_) => todo!(),
                        TermEntityPath::TypeOntology(path) => {
                            let mut arguments: SmallVec<[LocalTerm; 2]> = expansion
                                .arguments(db)
                                .iter()
                                .copied()
                                .map(Into::into)
                                .collect();
                            arguments.push(argument);
                            Ok(local_term_region.intern_unresolved_term(
                                src_expr_idx,
                                UnresolvedTerm::TypeOntology { path, arguments },
                            ))
                        }
                        TermEntityPath::TypeConstructor(_) => todo!(),
                    },
                    Term::Category(_) => todo!(),
                    Term::Universe(_) => todo!(),
                    Term::Curry(_) => todo!(),
                    Term::Ritchie(_) => todo!(),
                    Term::Abstraction(_) => todo!(),
                    Term::Application(_) => todo!(),
                    Term::Subentity(_) => todo!(),
                    Term::AsTraitSubentity(_) => todo!(),
                    Term::TraitConstraint(_) => todo!(),
                }
            }
            (LocalTerm::Unresolved(_), LocalTerm::Resolved(_)) => todo!(),
            (LocalTerm::Unresolved(_), LocalTerm::Unresolved(_)) => todo!(),
        }
    }

    pub(crate) fn new_ty_ontology_application(
        db: &dyn ExprTypeDb,
        unresolved_terms: &mut UnresolvedTerms,
        src_expr_idx: ExprIdx,
        path: TypePath,
        arguments: SmallVec<[LocalTerm; 2]>,
    ) -> Self {
        let mut resolved_arguments: SmallVec<[Term; 2]> = smallvec![];
        for argument in &arguments {
            match argument {
                LocalTerm::Resolved(argument) => resolved_arguments.push(*argument),
                LocalTerm::Unresolved(_) => break,
            }
        }
        if resolved_arguments.len() == arguments.len() {
            todo!()
        } else {
            unresolved_terms.intern_unresolved_term(
                src_expr_idx,
                UnresolvedTerm::TypeOntology { path, arguments },
            )
        }
    }

    pub(crate) fn unravel_borrow(
        self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> Self {
        match self.pattern(db, unresolved_terms) {
            LocalTermPattern::TypeOntology {
                refined_path: Right(PreludeTypePath::Borrow(path)),
                arguments,
                ..
            } => match path {
                PreludeBorrowTypePath::Ref | PreludeBorrowTypePath::RefMut => {
                    assert_eq!(arguments.len(), 2);
                    todo!()
                }
                PreludeBorrowTypePath::Leash => {
                    assert_eq!(arguments.len(), 1);
                    arguments[0]
                }
            },
            _ => self,
        }
    }

    fn resolved(self) -> Option<Term> {
        match self {
            LocalTerm::Resolved(term) => Some(term),
            LocalTerm::Unresolved(_) => None,
        }
    }

    pub(crate) fn resolve_progress(
        self,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermResolveResultRef<Self> {
        match self {
            LocalTerm::Resolved(term) => Ok(term.into()),
            LocalTerm::Unresolved(idx) => idx.resolve_progress(unresolved_terms),
        }
    }
}

impl From<TermLiteral> for LocalTerm {
    fn from(value: TermLiteral) -> Self {
        LocalTerm::Resolved(value.into())
    }
}
impl From<TermSymbol> for LocalTerm {
    fn from(value: TermSymbol) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermEntityPath> for LocalTerm {
    fn from(value: TermEntityPath) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCategory> for LocalTerm {
    fn from(value: TermCategory) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermUniverse> for LocalTerm {
    fn from(value: TermUniverse) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCurry> for LocalTerm {
    fn from(value: TermCurry) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermRitchie> for LocalTerm {
    fn from(value: TermRitchie) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAbstraction> for LocalTerm {
    fn from(value: TermAbstraction) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermApplication> for LocalTerm {
    fn from(value: TermApplication) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermSubentity> for LocalTerm {
    fn from(value: TermSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAsTraitSubentity> for LocalTerm {
    fn from(value: TermAsTraitSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermTraitConstraint> for LocalTerm {
    fn from(value: TermTraitConstraint) -> Self {
        LocalTerm::Resolved(value.into())
    }
}
