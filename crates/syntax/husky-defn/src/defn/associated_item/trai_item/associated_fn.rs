use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedFnNodeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub node_decl: TraitAssociatedFnNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitAssociatedFnDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub decl: TraitAssociatedFnDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_associated_fn_defn(
    _db: &dyn DefnDb,
    _decl: TraitAssociatedFnDecl,
) -> TraitAssociatedFnDefn {
    todo!()
}
