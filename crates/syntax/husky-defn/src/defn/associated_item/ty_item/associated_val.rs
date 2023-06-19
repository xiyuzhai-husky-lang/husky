use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedValDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub decl: TypeAssociatedValDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_val_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedValDecl,
) -> TypeAssociatedValDefn {
    todo!()
}
