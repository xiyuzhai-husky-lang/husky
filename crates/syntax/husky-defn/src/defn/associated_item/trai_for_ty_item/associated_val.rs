use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedValNodeDefn {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node_decl: TraitForTypeAssociatedValNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedValDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedValDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_val_defn(
    _db: &dyn DefnDb,
    _decl: TraitForTypeAssociatedValDecl,
) -> TraitForTypeAssociatedValDefn {
    todo!()
}
