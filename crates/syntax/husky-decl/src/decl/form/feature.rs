use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct FeatureDecl {
    #[id]
    pub path: FormPath,
    pub ast_idx: AstIdx,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolColonToken,
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
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectOutputType)?)
        } else {
            None
        };
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectEolColon)?;
        Ok(FeatureDecl::new(
            self.db(),
            path,
            ast_idx,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
        .into())
    }
}
