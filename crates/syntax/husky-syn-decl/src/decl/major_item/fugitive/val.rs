use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct MajorValSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    #[return_ref]
    pub colon_token: SynNodeDeclResult<ColonRegionalToken>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<ReturnTypeBeforeEqSyndicate>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl MajorValSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        chain_as_ref_err_collect!(self.colon_token(db), self.return_ty(db), self.eq_token(db))
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_val_node_decl(
        &self,
        syn_node_path: FugitiveSynNodePath,
    ) -> MajorValSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::False, AllowSelfValue::False, None);
        let colon_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedColonBeforeValReturnType);
        let return_ty = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedValReturnType);
        let eq_token = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectEqTokenForMemo);
        let expr = parser.parse_expr_root(None, SynExprRootKind::ValExpr);
        MajorValSynNodeDecl::new(
            self.db(),
            syn_node_path,
            colon_token,
            return_ty,
            eq_token,
            expr,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct MajorValSynDecl {
    #[id]
    pub path: FugitivePath,
    pub return_ty: ReturnTypeBeforeEqSyndicate,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl MajorValSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: FugitivePath,
        syn_node_decl: MajorValSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let val_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, val_ty, expr, syn_expr_region))
    }
}
