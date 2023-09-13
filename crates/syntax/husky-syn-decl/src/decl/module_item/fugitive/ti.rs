use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAliasSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAliasSynDecl {
    #[id]
    pub path: FugitivePath,
    pub syn_expr_region: SynExprRegion,
}
