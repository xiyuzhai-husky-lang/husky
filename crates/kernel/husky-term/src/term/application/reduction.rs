use super::*;

impl TermApplication {
    pub(super) fn reduce(self, db: &dyn TermDb) -> Term {
        reduce_term_application(db, self)
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn reduce_term_application(db: &dyn TermDb, term_application: TermApplication) -> Term {
    match term_application.function(db) {
        Term::Literal(_)
        | Term::Symbol(_)
        | Term::EntityPath(
            TermEntityPath::Trait(_)
            | TermEntityPath::TypeOntology(_)
            | TermEntityPath::TypeConstructor(_),
        ) => term_application.into(),
        Term::EntityPath(TermEntityPath::Form(_)) => todo!(),
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
