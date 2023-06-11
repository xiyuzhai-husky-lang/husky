use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedTypeDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TypeAssociatedTypeDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedTypeDecl,
) -> TypeAssociatedTypeDefn {
    todo!()
}
