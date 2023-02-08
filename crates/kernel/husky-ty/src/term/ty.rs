use super::*;

pub(crate) fn term_ty(db: &dyn TypeDb, term: Term) -> TypeResult<ReducedTerm> {
    match term {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Entity(_) => todo!(),
        Term::Category(cat) => cat
            .ty()
            .map(Into::into)
            .map(|term| reduced_term(db, term))
            .map_err(|e| OriginalTypeError::Term(e).into()),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Durant(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => todo!(),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}
