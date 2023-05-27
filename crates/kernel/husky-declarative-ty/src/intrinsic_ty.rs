use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct IntrinsicDeclarativeType(DeclarativeTerm);

impl IntrinsicDeclarativeType {
    pub fn reduced_declarative_term(self) -> DeclarativeTerm {
        self.0
    }
}

pub(crate) fn intrinsic_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    declarative_ty: DeclarativeTerm,
) -> IntrinsicDeclarativeType {
    match declarative_ty {
        DeclarativeTerm::Literal(_) => todo!(),
        DeclarativeTerm::Symbol(_) => todo!(),
        DeclarativeTerm::Entideclarative_tyPath(path) => match path {
            DeclarativeTermEntideclarative_tyPath::Form(_) => todo!(),
            DeclarativeTermEntideclarative_tyPath::Trait(_) => todo!(),
            DeclarativeTermEntideclarative_tyPath::DeclarativeTypeOntology(_) => todo!(),
            DeclarativeTermEntideclarative_tyPath::DeclarativeTypeConstructor(_) => todo!(),
        },
        // Entideclarative_tyPath::Module(_) => todo!(),
        // Entideclarative_tyPath::ModuleItem(path) => match path {
        //     ModuleItemPath::DeclarativeType(path) => IntrinsicDeclarativeType(declarative_ty),
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // Entideclarative_tyPath::AssociatedItem(_) => todo!(),
        // Entideclarative_tyPath::Variant(_) => todo!(),
        DeclarativeTerm::Category(_) => todo!(),
        DeclarativeTerm::Universe(_) => todo!(),
        DeclarativeTerm::Curry(_) => todo!(),
        DeclarativeTerm::Ritchie(_) => todo!(),
        DeclarativeTerm::Abstraction(_) => todo!(),
        DeclarativeTerm::Application(_) => todo!(),
        DeclarativeTerm::Subentideclarative_ty(_) => todo!(),
        DeclarativeTerm::AsTraitSubentideclarative_ty(_) => todo!(),
        DeclarativeTerm::TraitConstraint(_) => todo!(),
    }
}
