use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAliasDecl {
    #[id]
    pub path: FugitivePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {}
