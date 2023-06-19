use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAliasDefn {
    #[id]
    pub node_id: FugitiveNodeId,
    pub decl: TypeAliasDecl,
    pub expr_region: ExprRegion,
}
