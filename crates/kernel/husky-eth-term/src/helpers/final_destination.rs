use super::*;

impl EthTerm {
    pub fn final_destination(self, db: &::salsa::Db) -> FinalDestination {
        match self {
            EthTerm::Literal(_) => FinalDestination::AnyDerived,
            EthTerm::Symbol(_) | EthTerm::Rune(_) => FinalDestination::AnyOriginal,
            EthTerm::EntityPath(path) => match path {
                ItemPathTerm::Fugitive(_) => todo!(),
                ItemPathTerm::Trait(_) => todo!(),
                ItemPathTerm::TypeOntology(_) => FinalDestination::TypeOntology,
                ItemPathTerm::TypeInstance(_) => todo!(),
                ItemPathTerm::TypeVariant(_) => todo!(),
            },
            EthTerm::Category(_) => FinalDestination::Sort,
            EthTerm::Universe(_) => unreachable!("expect ty term"),
            EthTerm::Curry(slf) => curry_ethereal_term_final_destination(db, slf),
            EthTerm::Ritchie(slf) => FinalDestination::Ritchie(slf.ritchie_kind(db)),
            EthTerm::Abstraction(_) => unreachable!("expect ty term"),
            EthTerm::Application(slf) => application_ethereal_term_final_destination(db, slf),
            EthTerm::TypeAsTraitItem(_) => FinalDestination::AnyOriginal,
            EthTerm::TraitConstraint(_) => FinalDestination::Sort,
        }
    }
}

#[salsa::tracked(jar = EthTermJar)]
fn application_ethereal_term_final_destination(
    _db: &::salsa::Db,
    _term_application: EthApplication,
) -> FinalDestination {
    todo!()
}

#[salsa::tracked(jar = EthTermJar)]
fn curry_ethereal_term_final_destination(db: &::salsa::Db, curry: EthCurry) -> FinalDestination {
    curry.return_ty(db).final_destination(db)
}
