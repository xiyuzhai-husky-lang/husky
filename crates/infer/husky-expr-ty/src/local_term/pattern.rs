use super::*;

pub enum LocalTermPattern {
    Literal(TermLiteral),
    TypeOntologyPathApplication {
        path: Either<CustomTypePath, PreludeTypePath>,
        arguments: Vec<LocalTerm>,
    },
    TypeConstructorPathApplication {
        path: Either<CustomTypePath, PreludeTypePath>,
        arguments: Vec<LocalTerm>,
    },
    Curry {},
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

impl LocalTermPattern {
    fn from_resolved(db: &dyn ExprTypeDb, term: Term) -> Self {
        match term {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::EntityPath(_) => todo!(),
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

    fn from_unresolved(
        db: &dyn ExprTypeDb,
        term: UnresolvedTermIdx,
        unresolved_terms: &UnresolvedTerms,
    ) -> Self {
        match unresolved_terms[term].unresolved_term() {
            UnresolvedTerm::ImplicitSymbol(_) => todo!(),
            UnresolvedTerm::TypeApplication { ty_path, arguments } => todo!(),
            UnresolvedTerm::Ritchie {
                ritchie_kind,
                parameter_tys,
                return_ty,
            } => todo!(),
        }
    }
}
