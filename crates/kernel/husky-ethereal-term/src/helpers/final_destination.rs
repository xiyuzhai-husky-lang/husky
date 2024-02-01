use super::*;

impl EtherealTerm {
    pub fn final_destination(self, db: &::salsa::Db) -> FinalDestination {
        match self {
            EtherealTerm::Literal(_) => FinalDestination::AnyDerived,
            EtherealTerm::Symbol(_) | EtherealTerm::Rune(_) => FinalDestination::AnyOriginal,
            EtherealTerm::EntityPath(path) => match path {
                ItemPathTerm::Fugitive(_) => todo!(),
                ItemPathTerm::Trait(_) => todo!(),
                ItemPathTerm::TypeOntology(_) => FinalDestination::TypeOntology,
                ItemPathTerm::TypeInstance(_) => todo!(),
                ItemPathTerm::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Category(_) => FinalDestination::Sort,
            EtherealTerm::Universe(_) => unreachable!("expect ty term"),
            EtherealTerm::Curry(slf) => curry_ethereal_term_final_destination(db, slf),
            EtherealTerm::Ritchie(slf) => FinalDestination::Ritchie(slf.ritchie_kind(db)),
            EtherealTerm::Abstraction(_) => unreachable!("expect ty term"),
            EtherealTerm::Application(slf) => application_ethereal_term_final_destination(db, slf),
            EtherealTerm::TypeAsTraitItem(_) => FinalDestination::AnyOriginal,
            EtherealTerm::TraitConstraint(_) => FinalDestination::Sort,
        }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
fn application_ethereal_term_final_destination(
    _db: &::salsa::Db,
    _term_application: ApplicationEtherealTerm,
) -> FinalDestination {
    todo!()
}

#[salsa::tracked(jar = EtherealTermJar)]
fn curry_ethereal_term_final_destination(
    db: &::salsa::Db,
    curry: CurryEtherealTerm,
) -> FinalDestination {
    curry.return_ty(db).final_destination(db)
}
