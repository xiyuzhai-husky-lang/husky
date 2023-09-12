use super::*;
use husky_print_utils::p;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct ValSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub ast_idx: AstIdx,
    pub colon_token: TokenDataResult<Option<ColonToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeEqObelisk>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqToken>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl ValSynNodeDecl {
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
    pub(super) fn parse_val_node_decl(
        &self,
        syn_node_path: FugitiveSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> ValSynNodeDecl {
        let mut parser = self.parser(
            syn_node_path,
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
            None,
            token_group_idx,
            Some(saved_stream_state),
        );
        let colon_token = parser.try_parse_option();
        let var_ty = if let Ok(Some(_)) = colon_token {
            parser
                .try_parse_expected(OriginalSynNodeDeclError::ExpectedVariableType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eq_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectEqTokenForVariable);
        let expr = parser.parse_expr_root(None, ExprRootKind::ValExpr);
        ValSynNodeDecl::new(
            self.db(),
            syn_node_path,
            ast_idx,
            colon_token,
            var_ty,
            eq_token,
            expr,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct ValSynDecl {
    #[id]
    pub path: FugitivePath,
    pub return_ty: Option<ReturnTypeBeforeEqObelisk>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl ValSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: FugitivePath,
        syn_node_decl: ValSynNodeDecl,
    ) -> DeclResult<Self> {
        let val_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(ValSynDecl::new(db, path, val_ty, expr, syn_expr_region))
    }
}
