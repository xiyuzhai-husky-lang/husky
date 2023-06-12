use super::*;
use husky_print_utils::p;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct ValRawDecl {
    #[id]
    pub node_path: FugitiveNodePath,
    pub ast_idx: AstIdx,
    pub colon_token: Option<ColonToken>,
    pub var_ty: Option<FormTypeExpr>,
    pub eq_token: EqToken,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct ValDecl {
    #[id]
    pub node_path: FugitiveNodePath,
    pub ast_idx: AstIdx,
    pub colon_token: Option<ColonToken>,
    pub var_ty: Option<FormTypeExpr>,
    pub eq_token: EqToken,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_feature_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
        id: FugitiveNodePath,
    ) -> Result<FugitiveDecl, DeclError> {
        let mut parser = self.expr_parser(id, None, AllowSelfType::False, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let colon_token = ctx.parse()?;
        let form_ty = if colon_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectedVariableType)?)
        } else {
            None
        };
        let eq_token = ctx.parse_expected(OriginalDeclExprError::ExpectEqTokenForVariable)?;
        let expr = ctx.parse_expr_root(None, ExprRootKind::ValExpr);
        Ok(ValDecl::new(
            self.db(),
            id,
            ast_idx,
            colon_token,
            form_ty,
            eq_token,
            expr,
            parser.finish(),
        )
        .into())
    }
}
