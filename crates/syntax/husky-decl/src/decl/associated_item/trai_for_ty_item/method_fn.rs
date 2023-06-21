use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeMethodFnNodeDecl {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node: TraitForTypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    pub explicit_parameter_decl_list: DeclExprResult<ExplicitParameterDeclList>,
    #[return_ref]
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: DeclExprResult<Option<ReturnTypeExpr>>,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolToken>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_trai_for_ty_method_fn_node_decl(
        &self,
        node_path: TraitForTypeItemNodePath,
        node: TraitForTypeItemNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitForTypeMethodFnNodeDecl {
        let db = self.db();
        let impl_block_node_decl = node_path.impl_block(db).node_decl(db);
        let mut parser = self.expr_parser(
            node.node_path(db),
            Some(impl_block_node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        let parameter_decl_list =
            ctx.try_parse_expected(OriginalDeclExprError::ExpectedParameterDeclList);
        let curry_token = ctx.try_parse_optional();
        let return_ty = if let Ok(Some(_)) = curry_token {
            ctx.try_parse_expected(OriginalDeclExprError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = ctx.try_parse_expected(OriginalDeclExprError::ExpectedEolColon);
        TraitForTypeMethodFnNodeDecl::new(
            db,
            node.node_path(db),
            node,
            ast_idx,
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeMethodFnDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub self_parameter: Option<SelfParameterDeclPattern>,
    #[return_ref]
    pub regular_parameters: RegularParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeMethodFnDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitForTypeItemPath,
        node_decl: TraitForTypeMethodFnNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let explicit_parameter_decl_list = node_decl.explicit_parameter_decl_list(db).as_ref()?;
        let self_parameter = *explicit_parameter_decl_list.self_parameter();
        let regular_parameters: RegularParameterDeclPatterns = explicit_parameter_decl_list
            .regular_parameters()
            .to_smallvec();
        let return_ty = *node_decl.return_ty(db).as_ref()?;
        let expr_region = node_decl.expr_region(db);
        Ok(TraitForTypeMethodFnDecl::new(
            db,
            path,
            implicit_parameters,
            self_parameter,
            regular_parameters,
            return_ty,
            expr_region,
        ))
    }

    pub fn impl_block_path(self, db: &dyn DeclDb) -> TraitForTypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
