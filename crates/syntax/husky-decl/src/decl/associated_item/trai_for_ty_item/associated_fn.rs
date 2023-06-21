use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedFnNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedFnDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeAssociatedFnDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitForTypeItemPath,
        node_decl: TraitForTypeAssociatedFnNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
