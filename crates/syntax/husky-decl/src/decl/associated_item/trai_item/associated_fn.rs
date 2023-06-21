use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitAssociatedFnNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitAssociatedFnDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParser<'a> {}
