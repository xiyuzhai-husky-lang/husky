use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedTypeDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub decl: TypeAssociatedTypeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_ty_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedTypeDecl,
) -> TypeAssociatedTypeDefn {
    todo!()
}
