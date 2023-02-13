use super::*;

pub(crate) fn term_ty(db: &dyn TypeDb, reduced_term: ReducedTerm) -> TypeResult<ReducedTerm> {
    match reduced_term.term() {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Entity(_) => todo!(),
        Term::Category(cat) => cat
            .ty()
            .map(Into::into)
            .map(|term| calc_reduced_term(db, term))
            .map_err(|e| OriginalTypeError::Term(e).into()),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(term) => application_term_ty(db, term),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn application_term_ty(
    db: &dyn TypeDb,
    term: TermApplication,
) -> TypeResult<ReducedTerm> {
    todo!()
}
