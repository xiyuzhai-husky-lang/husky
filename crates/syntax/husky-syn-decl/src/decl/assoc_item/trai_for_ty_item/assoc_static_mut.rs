use super::*;

#[salsa::tracked]
pub struct TraitForTypeAssocStaticMutSynNodeDecl {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    #[return_ref]
    pub colon_token: SynNodeDeclResult<ColonRegionalToken>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<ReturnTypeBeforeEqSyndicate>,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked]
pub struct TraitForTypeAssocStaticMutSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ty_term_expr_idx: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}
