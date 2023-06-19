use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMemoizedFieldNodeDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub associated_item_node: AssociatedItemNode,
    pub ast_idx: AstIdx,
    pub colon_token: Option<ColonToken>,
    pub memo_ty: Option<FormTypeExpr>,
    pub eq_token: EqToken,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMemoizedFieldDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub ast_idx: AstIdx,
    pub colon_token: Option<ColonToken>,
    pub memo_ty: Option<FormTypeExpr>,
    pub eq_token: EqToken,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeMemoizedFieldDecl {
    pub fn impl_block_node_path(self, db: &dyn DeclDb) -> TypeImplBlockNodePath {
        self.node_path(db).impl_block(db)
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TypeItemNode,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TypeMemoizedFieldDecl> {
        let db = self.db();
        let node_path = node.node_path(db);
        let impl_block_node_decl = node_path.impl_block(db).node_decl(db);
        let mut parser = self.expr_parser(
            node_path,
            Some(impl_block_node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let colon_token = ctx.try_parse_optional()?;
        let form_ty = if colon_token.is_some() {
            Some(ctx.parse_expected(OriginalDeclExprError::ExpectedOutputType)?)
        } else {
            None
        };
        let eq_token = ctx.parse_expected(OriginalDeclExprError::ExpectEqTokenForVariable)?;
        let expr = ctx.parse_expr_root(None, ExprRootKind::ValExpr);
        Ok(TypeMemoizedFieldDecl::new(
            db,
            node_path,
            ast_idx,
            colon_token,
            form_ty,
            eq_token,
            expr,
            parser.finish(),
        )
        .into())
    }
}
