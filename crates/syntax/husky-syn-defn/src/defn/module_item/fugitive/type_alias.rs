use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAliasSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAliasSynDefn {
    #[id]
    pub path: FugitivePath,
    pub syn_expr_region: SynExprRegion,
}
