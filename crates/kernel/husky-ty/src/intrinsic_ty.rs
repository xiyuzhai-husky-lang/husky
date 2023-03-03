use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct IntrinsicType(ReducedTerm);

impl IntrinsicType {
    pub fn reduced_term(self) -> ReducedTerm {
        self.0
    }
}

pub(crate) fn intrinsic_ty(db: &dyn TypeDb, ty: ReducedTerm) -> IntrinsicType {
    match ty.term() {
        Term::Literal(_) => todo!(),
        Term::Symbol(_) => todo!(),
        Term::Entity(path) => match path {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(path) => match path {
                ModuleItemPath::Type(path) => IntrinsicType(ty),
                ModuleItemPath::Trait(_) => todo!(),
                ModuleItemPath::Form(_) => todo!(),
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        },
        Term::Category(_) => todo!(),
        Term::Universe(_) => todo!(),
        Term::Curry(_) => todo!(),
        Term::Ritchie(_) => todo!(),
        Term::Abstraction(_) => todo!(),
        Term::Application(_) => todo!(),
        Term::Composition(_) => todo!(),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}
