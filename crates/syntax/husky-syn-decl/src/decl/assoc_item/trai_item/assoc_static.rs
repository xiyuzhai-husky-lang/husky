use super::*;

#[salsa::tracked]
pub struct TraitAssocStaticSynNodeDecl {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    #[return_ref]
    pub colon_token: SynNodeDeclResult<ColonRegionalToken>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<ReturnTypeBeforeEqSyndicate>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitAssocStaticSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        chain_as_ref_err_collect!(self.colon_token(db), self.return_ty(db))
    }
}

/// # parse
impl<'a> ItemSynNodeDeclParser<'a> {
    pub(super) fn parse_trai_assoc_static_node_decl(
        &self,
        syn_node_path: TraitItemSynNodePath,
    ) -> TraitAssocStaticSynNodeDecl {
        let db = self.db();
        let parent_trai_syn_node_decl = syn_node_path
            .data(db)
            .parent_trai_syn_node_path
            .syn_node_decl(db);
        let mut parser = self.expr_parser(
            Some(parent_trai_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
        );
        let colon_token = parser
            .try_parse_expected(OriginalSynNodeDeclError::ExpectedColonBeforeStaticReturnType);
        let return_ty =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedStaticReturnType);
        TraitAssocStaticSynNodeDecl::new(db, syn_node_path, colon_token, return_ty, parser.finish())
    }
}

#[salsa::tracked]
pub struct TraitAssocStaticSynDecl {
    #[id]
    pub path: TraitItemPath,
    pub syn_expr_region: SynExprRegion,
}
