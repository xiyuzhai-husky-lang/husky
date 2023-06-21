use super::*;
use husky_print_utils::p;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct ValNodeDecl {
    #[id]
    pub node_path: FugitiveNodePath,
    pub ast_idx: AstIdx,
    pub colon_token: TokenResult<Option<ColonToken>>,
    #[return_ref]
    pub var_ty: DeclExprResult<Option<FormTypeExpr>>,
    #[return_ref]
    pub eq_token: DeclExprResult<EqToken>,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_val_node_decl(
        &self,
        node_path: FugitiveNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> ValNodeDecl {
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::False, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let colon_token = ctx.try_parse_optional();
        let var_ty = if let Ok(Some(_)) = colon_token {
            ctx.try_parse_expected(OriginalDeclExprError::ExpectedVariableType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eq_token = ctx.try_parse_expected(OriginalDeclExprError::ExpectEqTokenForVariable);
        let expr = ctx.parse_expr_root(None, ExprRootKind::ValExpr);
        ValNodeDecl::new(
            self.db(),
            node_path,
            ast_idx,
            colon_token,
            var_ty,
            eq_token,
            expr,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct ValDecl {
    #[id]
    pub path: FugitivePath,
    pub var_ty: Option<FormTypeExpr>,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}
