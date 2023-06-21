use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedFnNodeDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    pub parameter_decl_list: DeclExprResult<ExplicitParameterDeclList>,
    #[return_ref]
    pub curry_token: DeclExprResult<Option<CurryToken>>,
    pub return_ty: Option<ReturnTypeExpr>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolToken>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAssociatedFnDecl {
    #[id]
    pub path: TypeItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub parameter_decl_list: ExplicitParameterDeclList,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl TypeAssociatedFnDecl {
    pub fn parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.parameter_decl_list(db).regular_parameters()
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_ty_associated_fn_node_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TypeItemNode,
        saved_stream_state: TokenStreamState,
    ) -> TypeAssociatedFnNodeDecl {
        todo!()
        // let db = self.db();
        // let impl_block_node_decl = node.impl_block_node_path(db).node_decl(db);
        // let node_path = node.node_path(db);
        // let mut parser = self.expr_parser(
        //     node_path,
        //     Some(impl_block_node_decl.expr_region(db)),
        //     AllowSelfType::True,
        //     AllowSelfValue::True,
        // );
        // let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
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
        // Ok(TypeAssociatedFnDecl::new(
        //     db,
        //     node_path,
        //     node,
        //     ast_idx,
        //     implicit_parameter_decl_list,
        //     parameter_decl_list,
        //     curry_token,
        //     return_ty,
        //     eol_colon,
        //     parser.finish(),
        // ))
    }
}
