use super::*;

#[salsa::tracked]
pub struct TraitMemoizedFieldSynNodeDecl {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    #[return_ref]
    pub colon_token: SynNodeDeclResult<ColonRegionalToken>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<ReturnTypeBeforeEqSyndicate>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMemoizedFieldSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.colon_token(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.return_ty(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_trai_memo_syn_node_decl(
        &self,
        syn_node_path: TraitItemSynNodePath,
    ) -> TraitMemoizedFieldSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = syn_node_path
            .data(db)
            .parent_trai_syn_node_path
            .syn_node_decl(db);
        let mut parser = self.expr_parser(
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
            None,
        );
        let colon_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedColonForTraitMemoizedField);
        let return_ty = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedOutputType);
        TraitMemoizedFieldSynNodeDecl::new(
            db,
            syn_node_path,
            colon_token,
            return_ty,
            parser.finish(),
        )
    }
}

#[salsa::tracked]
pub struct TraitMemoizedFieldSynDecl {
    #[id]
    pub path: TypeItemPath,
    pub return_ty: Option<ReturnTypeBeforeEqSyndicate>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMemoizedFieldSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        syn_node_decl: TypeMemoizedFieldSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, return_ty, expr, syn_expr_region))
    }
}
