use super::*;
use husky_print_utils::p;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct ValFugitiveSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub colon_token: TokenDataResult<Option<ColonRegionalToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeEqSyndicate>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl ValFugitiveSynNodeDecl {
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

impl<'a> DeclParser<'a, FugitiveSynNodePath> {
    pub(super) fn parse_val_node_decl(&self) -> ValFugitiveSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::False, AllowSelfValue::False, None);
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
        let expr = parser.parse_expr_root(None, SynExprRootKind::ValExpr);
        ValFugitiveSynNodeDecl::new(
            self.db(),
            self.syn_node_path(),
            colon_token,
            var_ty,
            eq_token,
            expr,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct ValFugitiveSynDecl {
    #[id]
    pub path: FugitivePath,
    pub return_ty: Option<ReturnTypeBeforeEqSyndicate>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl ValFugitiveSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: FugitivePath,
        syn_node_decl: ValFugitiveSynNodeDecl,
    ) -> DeclResult<Self> {
        let val_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(ValFugitiveSynDecl::new(
            db,
            path,
            val_ty,
            expr,
            syn_expr_region,
        ))
    }
}
