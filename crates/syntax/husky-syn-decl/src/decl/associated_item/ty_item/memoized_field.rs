use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeMemoizedFieldSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub ast_idx: AstIdx,
    pub colon_token: TokenResult<Option<ColonToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeEqObelisk>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqToken>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMemoizedFieldSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.return_ty(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eq_token(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParserFactory<'a> {
    pub(super) fn parse_ty_memo_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        node: TypeItemSynNode,
        saved_stream_state: TokenStreamState,
    ) -> TypeMemoizedFieldSynNodeDecl {
        let db = self.db();
        let syn_node_path = node.syn_node_path(db);
        let impl_block_syn_node_decl = syn_node_path.impl_block(db).syn_node_decl(db);
        let mut parser = self.expr_parser(
            syn_node_path,
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let colon_token = ctx.try_parse_option();
        let form_ty = if let Ok(Some(_)) = colon_token {
            ctx.try_parse_expected(OriginalSynNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eq_token = ctx.try_parse_expected(OriginalSynNodeDeclError::ExpectEqTokenForVariable);
        let expr = ctx.parse_expr_root(None, ExprRootKind::ValExpr);
        TypeMemoizedFieldSynNodeDecl::new(
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

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeMemoizedFieldSynDecl {
    #[id]
    pub path: TypeItemPath,
    pub return_ty: Option<ReturnTypeBeforeEqObelisk>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMemoizedFieldSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeItemPath,
        syn_node_decl: TypeMemoizedFieldSynNodeDecl,
    ) -> DeclResult<Self> {
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, return_ty, expr, syn_expr_region))
    }

    pub fn impl_block_path(self, db: &dyn SynDeclDb) -> TypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
