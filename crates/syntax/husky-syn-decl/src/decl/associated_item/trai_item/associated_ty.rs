use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedTypeSynNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedTypeSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}
