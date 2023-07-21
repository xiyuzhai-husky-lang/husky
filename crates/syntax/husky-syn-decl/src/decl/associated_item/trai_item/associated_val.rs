use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedValNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedValDecl {
    #[id]
    pub path: TraitItemPath,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParser<'a> {}
