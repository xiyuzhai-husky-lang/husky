use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitMethodFnNodeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub node_decl: TraitMethodFnNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitMethodFnDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitMethodFnDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_method_defn(_db: &dyn DefnDb, _decl: TraitMethodFnDecl) -> TraitMethodFnDefn {
    todo!()
}
