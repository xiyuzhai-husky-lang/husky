use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMethodFnNodeDecl {
    #[id]
    pub node_id: TypeItemNodeId,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub explicit_parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMethodFnDecl {
    #[id]
    pub node_id: TypeItemNodeId,
    pub node: TypeItemNode,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub explicit_parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
}

impl TypeMethodFnDecl {
    pub fn self_parameter<'a>(self, db: &'a dyn DeclDb) -> Option<&'a SelfParameterDeclPattern> {
        self.explicit_parameter_decl_list(db)
            .self_parameter()
            .as_ref()
    }

    pub fn regular_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [RegularParameterDeclPattern] {
        self.explicit_parameter_decl_list(db).regular_parameters()
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }

    pub fn impl_block_node_id(self, db: &dyn DeclDb) -> TypeImplBlockNodeId {
        self.node_id(db).impl_block(db)
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_method_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TypeItemNode,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TypeMethodFnDecl> {
        let db = self.db();
        let impl_block_node_decl = node.impl_block(db).node_decl(db);
        let mut parser = self.expr_parser(
            node.node_id(db),
            Some(impl_block_node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let implicit_parameter_decl_list = ctx.parse()?;
        let parameter_decl_list =
            ctx.parse_expected(OriginalDeclExprError::ExpectedParameterDeclList)?;

        let curry_token = ctx.parse()?;
        let return_ty = if curry_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectedOutputType)?)
        } else {
            None
        };
        let eol_colon = ctx.parse_expected(OriginalDeclExprError::ExpectedEolColon)?;
        Ok(TypeMethodFnDecl::new(
            self.db(),
            node.node_id(self.db()),
            node,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
        .into())
    }
}
