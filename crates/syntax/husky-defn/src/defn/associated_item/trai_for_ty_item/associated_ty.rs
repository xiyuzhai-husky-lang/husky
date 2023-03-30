use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedTypeDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TraitForTypeAssociatedTypeDecl,
}

impl HasDefn for TraitForTypeAssociatedTypeDecl {
    type Defn = TraitForTypeAssociatedTypeDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        trai_for_ty_associated_ty_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TraitForTypeAssociatedTypeDecl,
) -> TraitForTypeAssociatedTypeDefn {
    todo!()
}
