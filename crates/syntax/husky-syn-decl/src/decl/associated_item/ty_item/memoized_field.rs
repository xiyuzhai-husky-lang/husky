use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeMemoizedFieldSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub colon_token: TokenDataResult<Option<ColonRegionalToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeEqSyndicate>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMemoizedFieldSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.return_ty(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eq_token(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a, TypeItemSynNodePath> {
    pub(super) fn parse_ty_memo_decl(&self) -> TypeMemoizedFieldSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = self.syn_node_path().impl_block(db).syn_node_decl(db);
        let mut parser = self.expr_parser(
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
            None,
        );
        let colon_token = parser.try_parse_option();
        let form_ty = if let Ok(Some(_)) = colon_token {
            parser
                .try_parse_expected(OriginalSynNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eq_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectEqTokenForVariable);
        let expr = parser.parse_expr_root(None, SynExprRootKind::ValExpr);
        TypeMemoizedFieldSynNodeDecl::new(
            db,
            self.syn_node_path(),
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
    pub return_ty: Option<ReturnTypeBeforeEqSyndicate>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMemoizedFieldSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        syn_node_decl: TypeMemoizedFieldSynNodeDecl,
    ) -> DeclResult<Self> {
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, return_ty, expr, syn_expr_region))
    }

    pub fn impl_block_path(self, db: &::salsa::Db) -> TypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
