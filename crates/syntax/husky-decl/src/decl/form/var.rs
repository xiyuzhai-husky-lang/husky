use husky_print_utils::p;

use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct ValDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub colon_token: Option<ColonToken>,
    pub var_ty: Option<FormTypeExpr>,
    pub eq_token: EqToken,
    pub expr_or_eol_token: Either<EolToken, ExprIdx>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_feature_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<FormDecl, DeclError> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let colon_token = ctx.parse()?;
        let form_ty = if colon_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectVariableType)?)
        } else {
            None
        };
        let eq_token = ctx.parse_expected(OriginalDeclExprError::ExpectEqTokenForVariable)?;
        let expr_or_eol_token = if let Some(eol_token) = ctx.parse::<EolToken>()? {
            Left(eol_token)
        } else {
            Right(todo!("parse expr"))
        };
        Ok(ValDecl::new(
            self.db(),
            path,
            ast_idx,
            colon_token,
            form_ty,
            eq_token,
            expr_or_eol_token,
            parser.finish(),
        )
        .into())
    }
}
