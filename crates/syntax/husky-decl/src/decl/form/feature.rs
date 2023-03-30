use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct FeatureDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub curry_token: DeclExprResult<CurryToken>,
    #[return_ref]
    pub return_ty: DeclExprResult<OutputTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {}
