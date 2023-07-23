use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedValSynNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedValSynDecl {
    #[id]
    pub path: TraitItemPath,
    pub syn_expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}
