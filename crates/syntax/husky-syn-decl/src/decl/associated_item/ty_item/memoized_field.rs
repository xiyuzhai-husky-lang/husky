use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeMemoizedFieldNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub ast_idx: AstIdx,
    pub colon_token: TokenResult<Option<ColonToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeExprBeforeEq>>,
    #[return_ref]
    pub eq_token: NodeDeclResult<EqToken>,
    pub expr: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TypeMemoizedFieldNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.return_ty(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eq_token(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TypeItemSynNode,
        saved_stream_state: TokenStreamState,
    ) -> TypeMemoizedFieldNodeDecl {
        let db = self.db();
        let syn_node_path = node.syn_node_path(db);
        let impl_block_syn_node_decl = syn_node_path.impl_block(db).node_decl(db);
        let mut parser = self.expr_parser(
            syn_node_path,
            Some(impl_block_syn_node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let colon_token = ctx.try_parse_option();
        let form_ty = if let Ok(Some(_)) = colon_token {
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eq_token = ctx.try_parse_expected(OriginalNodeDeclError::ExpectEqTokenForVariable);
        let expr = ctx.parse_expr_root(None, ExprRootKind::ValExpr);
        TypeMemoizedFieldNodeDecl::new(
            db,
            syn_node_path,
            ast_idx,
            colon_token,
            form_ty,
            eq_token,
            expr,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeMemoizedFieldDecl {
    #[id]
    pub path: TypeItemPath,
    pub return_ty: Option<ReturnTypeExprBeforeEq>,
    pub expr: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TypeMemoizedFieldDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeItemPath,
        node_decl: TypeMemoizedFieldNodeDecl,
    ) -> DeclResult<Self> {
        let return_ty = *node_decl.return_ty(db).as_ref()?;
        let expr = node_decl.expr(db);
        let expr_region = node_decl.expr_region(db);
        Ok(Self::new(db, path, return_ty, expr, expr_region))
    }

    pub fn impl_block_path(self, db: &dyn DeclDb) -> TypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
