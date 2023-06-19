use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeMethodFnNodeDecl {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node: TraitForTypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub explicit_parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeMethodFnDecl {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node: TraitForTypeItemNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    #[return_ref]
    pub explicit_parameter_decl_list: ExplicitParameterDeclList,
    pub curry_token: Option<CurryToken>,
    pub return_ty: Option<ReturnTypeExpr>,
    pub eol_colon: EolToken,
    pub expr_region: ExprRegion,
}

impl TraitForTypeMethodFnDecl {
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

    pub fn impl_block(self, db: &dyn DeclDb) -> TraitForTypeImplBlockNode {
        self.node(db).impl_block(db)
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_trai_for_ty_method_fn_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TraitForTypeItemNode,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TraitForTypeMethodFnDecl> {
        let db = self.db();
        let Ok(impl_decl) = node.impl_block(db).decl(db) else {
            return Err(
                DerivedDeclError::UnableToParseImplDeclForTyAsTraitMethodFnDecl.into()
            )
        };
        let mut parser = self.expr_parser(
            node.node_path(db),
            Some(impl_decl.expr_region(db)),
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
        Ok(TraitForTypeMethodFnDecl::new(
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
        ))
    }
}
