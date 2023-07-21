use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAliasNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeAliasDecl {
    #[id]
    pub path: FugitivePath,
    pub expr_region: SynExprRegion,
}
