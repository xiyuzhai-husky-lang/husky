use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedTypeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub decl: TraitAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedTypeDecl,
) -> TraitAssociatedTypeDefn {
    todo!()
}
