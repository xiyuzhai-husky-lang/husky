use super::*;

pub(super) fn ethereal_term_data<'a>(
    db: &'a dyn FluffyTermDb,
    term: EtherealTerm,
) -> FluffyTermData<'a> {
    match term {
        EtherealTerm::Literal(_) => todo!(),
        EtherealTerm::Symbol(term) => FluffyTermData::Symbol {
            ty: term.ty(db).into(),
        },
        EtherealTerm::Variable(term) => FluffyTermData::Variable {
            ty: term.ty(db).into(),
        },
        EtherealTerm::EntityPath(path) => match path {
            TermEntityPath::Fugitive(_) => todo!(),
            TermEntityPath::Trait(_) => todo!(),
            TermEntityPath::TypeOntology(ty_path) => FluffyTermData::TypeOntology {
                path: ty_path,
                refined_path: ty_path.refine(db),
                arguments: &[],
                ty_ethereal_term: Some(path.into()),
            },
            TermEntityPath::TypeInstance(_) => todo!(),
        },
        EtherealTerm::Category(term) => FluffyTermData::Category(term),
        EtherealTerm::Universe(_) => todo!(),
        EtherealTerm::Curry(term) => FluffyTermData::Curry {
            curry_kind: term.curry_kind(db),
            variance: term.variance(db),
            parameter_variable: term.parameter_variable(db).map(Into::into),
            parameter_ty: term.parameter_ty(db).into(),
            return_ty: term.return_ty(db).into(),
            ty_ethereal_term: Some(term),
        },
        EtherealTerm::Ritchie(term) => term_ritchie_fluffy_data(db, term).as_ref(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(term) => term_application_fluffy_data(db, term).as_ref(),
        EtherealTerm::Subentity(_) => todo!(),
        EtherealTerm::AsTraitSubentity(_) => todo!(),
        EtherealTerm::TraitConstraint(_) => todo!(),
    }
}
