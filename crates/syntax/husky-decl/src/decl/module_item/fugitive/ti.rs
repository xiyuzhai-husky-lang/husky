use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAliasNodeDecl {
    #[id]
    pub node_path: FugitiveNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAliasDecl {
    #[id]
    pub path: FugitivePath,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {}
