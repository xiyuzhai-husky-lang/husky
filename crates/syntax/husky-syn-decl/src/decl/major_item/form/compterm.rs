use super::*;
use either::*;

#[salsa::tracked]
pub struct MajorComptermSynNodeDecl {
    #[id]
    pub syn_node_path: MajorFormSynNodePath,
    #[return_ref]
    pub colon_token: SynNodeDeclResult<ColonRegionalToken>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<ReturnTypeBeforeEqSyndicate>,
    #[return_ref]
    pub eq_or_eol_semicolon_token:
        SynNodeDeclResult<Either<EqRegionalToken, EolSemicolonRegionalToken>>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl MajorComptermSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        chain_as_ref_err_collect!(
            self.colon_token(db),
            self.return_ty(db),
            self.eq_or_eol_semicolon_token(db)
        )
    }
}

impl<'a> ItemSynNodeDeclParser<'a> {
    pub(super) fn parse_termic_syn_node_decl(
        &self,
        syn_node_path: MajorFormSynNodePath,
    ) -> MajorComptermSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::False, AllowSelfValue::False, None);
        let colon_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedColonBeforeValReturnType);
        let return_ty = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedValReturnType);
        let eq_or_eol_semicolon_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEqTokenForMemo);
        let expr = parser.parse_expr_root(None, SynExprRootKind::ValExpr);
        MajorComptermSynNodeDecl::new(
            self.db(),
            syn_node_path,
            colon_token,
            return_ty,
            eq_or_eol_semicolon_token,
            expr,
            parser.finish(),
        )
    }
}

#[salsa::tracked]
pub struct MajorComptermSynDecl {
    #[id]
    pub path: MajorFormPath,
    pub return_ty: ReturnTypeBeforeEqSyndicate,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl MajorComptermSynDecl {
    pub(super) fn from_node(
        db: &::salsa::Db,
        path: MajorFormPath,
        syn_node_decl: MajorComptermSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let val_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, val_ty, expr, syn_expr_region))
    }
}
