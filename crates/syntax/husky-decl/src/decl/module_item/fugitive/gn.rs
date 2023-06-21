use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct GnNodeDecl {
    #[id]
    pub node_path: FugitiveNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

impl GnNodeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }

    pub fn parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.parameter_decl_list(db).regular_parameters()
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_gn_node_decl(
        &self,
        node_path: FugitiveNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> GnNodeDecl {
        todo!()
        // let mut parser = self.expr_parser(id, None, AllowSelfType::False, AllowSelfValue::False);
        // let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        // let implicit_parameter_decl_list = ctx.try_parse_optional()?;
        // let parameter_decl_list =
        //     ctx.parse_expected(OriginalDeclExprError::ExpectedParameterDeclList)?;

        // let curry_token = ctx.try_parse_optional()?;
        // let return_ty = if curry_token.is_some() {
        //     Some(ctx.parse_expected(OriginalDeclExprError::ExpectedOutputType)?)
        // } else {
        //     None
        // };
        // let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon)?;
        // Ok(GnDecl::new(
        //     self.db(),
        //     id,
        //     ast_idx,
        //     parser.finish(),
        //     implicit_parameter_decl_list,
        //     parameter_decl_list,
        //     curry_token,
        //     return_ty,
        //     eol_colon,
        // )
        // .into())
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct GnDecl {
    #[id]
    pub path: FugitivePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub regular_parameters: RegularParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl GnDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: FugitivePath,
        node_decl: GnNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
