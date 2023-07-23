use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAliasSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAliasSynDecl {
    #[id]
    pub path: FugitivePath,
    pub syn_expr_region: SynExprRegion,
}
