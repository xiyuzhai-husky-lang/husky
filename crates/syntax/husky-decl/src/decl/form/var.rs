use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct FeatureDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub colon_token: Option<ColonToken>,
    pub return_ty: Option<ReturnTypeExpr>,
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

        let curry_token = ctx.parse()?;
        let return_ty = if curry_token.is_some() {
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
        Ok(FeatureDecl::new(
            self.db(),
            path,
            ast_idx,
            curry_token,
            return_ty,
            eq_token,
            expr_or_eol_token,
            parser.finish(),
        )
        .into())
    }
}
