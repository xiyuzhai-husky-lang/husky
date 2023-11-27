use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitAssociatedTypeSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitAssociatedTypeSynNodeDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        syn_node_path: TraitItemSynNodePath,
        syn_node_decl: TraitAssociatedTypeSynNodeDecl,
    ) -> Self {
        TraitAssociatedTypeSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path.into(),
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::True,
                AllowSelfValue::False,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeSynDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitAssociatedTypeSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}
