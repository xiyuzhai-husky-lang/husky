use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMemoizedFieldNodeDecl {
    #[id]
    pub node_path: TypeItemNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub colon_token: TokenResult<Option<ColonToken>>,
    #[return_ref]
    pub memo_ty: DeclExprResult<Option<FormTypeExpr>>,
    #[return_ref]
    pub eq_token: DeclExprResult<EqToken>,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TypeItemNode,
        saved_stream_state: TokenStreamState,
    ) -> TypeMemoizedFieldNodeDecl {
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
        let colon_token = ctx.try_parse_optional();
        let form_ty = if let Ok(Some(_)) = colon_token {
            ctx.try_parse_expected(OriginalDeclExprError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eq_token = ctx.try_parse_expected(OriginalDeclExprError::ExpectEqTokenForVariable);
        let expr = ctx.parse_expr_root(None, ExprRootKind::ValExpr);
        TypeMemoizedFieldNodeDecl::new(
            db,
            node_path,
            ast_idx,
            colon_token,
            form_ty,
            eq_token,
            expr,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeMemoizedFieldDecl {
    #[id]
    pub path: TypeItemPath,
    pub memo_ty: Option<FormTypeExpr>,
    pub expr: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeMemoizedFieldDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeItemPath,
        node_decl: TypeMemoizedFieldNodeDecl,
    ) -> DeclResult<Self> {
        let memo_ty = *node_decl.memo_ty(db).as_ref()?;
        let expr = node_decl.expr(db);
        let expr_region = node_decl.expr_region(db);
        Ok(Self::new(db, path, memo_ty, expr, expr_region))
    }

    pub fn impl_block_path(self, db: &dyn DeclDb) -> TypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
