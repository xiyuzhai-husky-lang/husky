use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAliasSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TypeAliasSynDefn {
    #[id]
    pub path: FugitivePath,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}
