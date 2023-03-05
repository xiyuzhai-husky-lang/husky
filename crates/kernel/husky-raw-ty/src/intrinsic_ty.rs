use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct IntrinsicRawType(RawTerm);

impl IntrinsicRawType {
    pub fn reduced_raw_term(self) -> RawTerm {
        self.0
    }
}

pub(crate) fn intrinsic_raw_ty(db: &dyn RawTypeDb, raw_ty: RawTerm) -> IntrinsicRawType {
    match raw_ty {
        RawTerm::Literal(_) => todo!(),
        RawTerm::Symbol(_) => todo!(),
        RawTerm::Entiraw_tyPath(path) => match path {
            RawTermEntiraw_tyPath::Form(_) => todo!(),
            RawTermEntiraw_tyPath::Trait(_) => todo!(),
            RawTermEntiraw_tyPath::RawTypeOntology(_) => todo!(),
            RawTermEntiraw_tyPath::RawTypeConstructor(_) => todo!(),
        },
        // Entiraw_tyPath::Module(_) => todo!(),
        // Entiraw_tyPath::ModuleItem(path) => match path {
        //     ModuleItemPath::RawType(path) => IntrinsicRawType(raw_ty),
        //     ModuleItemPath::Trait(_) => todo!(),
        //     ModuleItemPath::Form(_) => todo!(),
        // },
        // Entiraw_tyPath::AssociatedItem(_) => todo!(),
        // Entiraw_tyPath::Variant(_) => todo!(),
        RawTerm::Category(_) => todo!(),
        RawTerm::Universe(_) => todo!(),
        RawTerm::Curry(_) => todo!(),
        RawTerm::Ritchie(_) => todo!(),
        RawTerm::Abstraction(_) => todo!(),
        RawTerm::Application(_) => todo!(),
        RawTerm::Subentiraw_ty(_) => todo!(),
        RawTerm::AsTraitSubentiraw_ty(_) => todo!(),
        RawTerm::TraitConstraint(_) => todo!(),
    }
}
