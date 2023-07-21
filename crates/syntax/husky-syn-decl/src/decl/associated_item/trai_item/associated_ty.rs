use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedTypeNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedTypeDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}
