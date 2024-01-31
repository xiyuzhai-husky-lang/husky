use super::*;

impl EtherealTerm {
    pub fn final_destination(self, _db: &::salsa::Db) -> FinalDestination {
        match self {
            EtherealTerm::Literal(_) => FinalDestination::AnyDerived,
            EtherealTerm::Symbol(_) | EtherealTerm::Rune(_) => FinalDestination::AnyOriginal,
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(_) => FinalDestination::TypeOntology,
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Category(_) => FinalDestination::Sort,
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::TypeAsTraitItem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
fn ethereal_term_application_final_destination(
    _db: &::salsa::Db,
    _term_application: EtherealTermApplication,
) -> FinalDestination {
    todo!()
}

#[salsa::tracked(jar = EtherealTermJar)]
fn ethereal_term_curry_final_destination(
    _db: &::salsa::Db,
    _term_curry: EtherealTermCurry,
) -> FinalDestination {
    todo!()
}
