use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMethodFnNodeDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
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

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_method_node_decl(
        &self,
        node_path: TypeItemNodePath,
        node: TypeItemNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TypeMethodFnNodeDecl {
        let db = self.db();
        let impl_block_node_decl = node.impl_block_node_path(db).node_decl(db);
        let mut parser = self.expr_parser(
            node.node_path(db),
            Some(impl_block_node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectedParameterDeclList);
        let curry_token = ctx.try_parse_optional();
        let return_ty = if let Ok(Some(_)) = curry_token {
            ctx.parse_expected(OriginalDeclExprError::ExpectedOutputType)
                .map(|return_ty| Some(return_ty))
        } else {
            Ok(None)
        };
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon);
        TypeMethodFnNodeDecl::new(
            db,
            node_path,
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
pub struct TypeMethodFnDecl {
    #[id]
    pub path: TypeItemPath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub explicit_parameter_decl_list: ExplicitParameterDeclList,
    pub return_ty: Option<ReturnTypeExpr>,
    pub expr_region: ExprRegion,
}

impl TypeMethodFnDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeItemPath,
        node_decl: TypeMethodFnNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(TypeMethodFnDecl::new(
            db,
            path,
            implicit_parameters,
            todo!(),
            todo!(),
            expr_region,
        ))
    }

    pub fn self_parameter<'a>(self, db: &'a dyn DeclDb) -> Option<&'a SelfParameterDeclPattern> {
        self.explicit_parameter_decl_list(db)
            .self_parameter()
            .as_ref()
    }

    pub fn regular_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.explicit_parameter_decl_list(db).regular_parameters()
    }

    pub fn impl_block_path(self, db: &dyn DeclDb) -> TypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
