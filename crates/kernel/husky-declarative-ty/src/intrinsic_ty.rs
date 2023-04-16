use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct IntrinsicDeclarativeType(DeclarativeTerm);

impl IntrinsicDeclarativeType {
    pub fn reduced_raw_term(self) -> DeclarativeTerm {
        self.0
    }
}

pub(crate) fn intrinsic_raw_ty(
    db: &dyn DeclarativeTypeDb,
    raw_ty: DeclarativeTerm,
) -> IntrinsicDeclarativeType {
    match raw_ty {
        DeclarativeTerm::Literal(_) => todo!(),
        DeclarativeTerm::Symbol(_) => todo!(),
        DeclarativeTerm::Entiraw_tyPath(path) => match path {
            DeclarativeTermEntiraw_tyPath::Form(_) => todo!(),
            DeclarativeTermEntiraw_tyPath::Trait(_) => todo!(),
            DeclarativeTermEntiraw_tyPath::DeclarativeTypeOntology(_) => todo!(),
            DeclarativeTermEntiraw_tyPath::DeclarativeTypeConstructor(_) => todo!(),
        },
        // Entiraw_tyPath::Module(_) => todo!(),
        // Entiraw_tyPath::ModuleItem(path) => match path {
        //     ModuleItemPath::DeclarativeType(path) => IntrinsicDeclarativeType(raw_ty),
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // Entiraw_tyPath::AssociatedItem(_) => todo!(),
        // Entiraw_tyPath::Variant(_) => todo!(),
        DeclarativeTerm::Category(_) => todo!(),
        DeclarativeTerm::Universe(_) => todo!(),
        DeclarativeTerm::Curry(_) => todo!(),
        DeclarativeTerm::Ritchie(_) => todo!(),
        DeclarativeTerm::Abstraction(_) => todo!(),
        DeclarativeTerm::Application(_) => todo!(),
        DeclarativeTerm::Subentiraw_ty(_) => todo!(),
        DeclarativeTerm::AsTraitSubentiraw_ty(_) => todo!(),
        DeclarativeTerm::TraitConstraint(_) => todo!(),
    }
}
