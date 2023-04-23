use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FluffyTraitForTypeMethodDisambiguation {
    indirections: SmallVec<[FluffyMethodIndirection; 2]>,
    ty_path: TypePath,
    trai_path: TraitPath,
    trai: FluffyTerm,
}

impl FluffyTraitForTypeMethodDisambiguation {
    pub fn indirections(&self) -> &[FluffyMethodIndirection] {
        &self.indirections
    }

    pub fn ty_path(&self) -> TypePath {
        self.ty_path
    }

    pub fn trai_path(&self) -> TraitPath {
        self.trai_path
    }

    pub fn trai(&self) -> FluffyTerm {
        self.trai
    }
}

impl FluffyTerm {
    pub(super) fn trai_for_ty_method_ty(
        self,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FluffyTermResult<
        Option<(
            FluffyTraitForTypeMethodDisambiguation,
            FluffyTermResult<FluffyTerm>,
        )>,
    > {
        for trai in available_traits {
            todo!()
        }
        Ok(None)
    }
}
