use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedValNodeDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node_decl: TypeAssociatedValNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedValDefn {
    #[id]
    pub path: TypeItemPath,
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
