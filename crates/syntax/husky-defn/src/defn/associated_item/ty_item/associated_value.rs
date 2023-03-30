use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedValueDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TypeAssociatedValueDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_value_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedValueDecl,
) -> TypeAssociatedValueDefn {
    todo!()
}
