use super::*;

impl EtherealTerm {
    pub fn final_destination(self, db: &::salsa::Db) -> FinalDestination {
        match self {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(_) => todo!(),
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(_) => FinalDestination::TypeOntology,
                TermEntityPath::TypeInstance(_) => todo!(),
                TermEntityPath::TypeVariant(_) => todo!(),
            },
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => todo!(),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
fn ethereal_term_application_final_destination(
    db: &::salsa::Db,
    term_application: EtherealTermApplication,
) -> FinalDestination {
    todo!()
}

#[salsa::tracked(jar = EtherealTermJar)]
fn ethereal_term_curry_final_destination(
    db: &::salsa::Db,
    term_curry: EtherealTermCurry,
) -> FinalDestination {
    todo!()
}
