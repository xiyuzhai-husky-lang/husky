use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedTypeDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TraitAssociatedTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedTypeDecl,
) -> TraitAssociatedTypeDefn {
    todo!()
}
