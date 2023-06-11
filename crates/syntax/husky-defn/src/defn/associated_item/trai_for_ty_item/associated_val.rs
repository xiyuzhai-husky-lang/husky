use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedValDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TraitForTypeAssociatedValDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_val_defn(
    _db: &dyn DefnDb,
    _decl: TraitForTypeAssociatedValDecl,
) -> TraitForTypeAssociatedValDefn {
    todo!()
}
