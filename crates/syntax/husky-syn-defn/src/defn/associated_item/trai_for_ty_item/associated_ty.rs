use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeAssociatedTypeSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitForTypeAssociatedTypeSynNodeDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        syn_node_path: TraitForTypeItemSynNodePath,
        syn_node_decl: TraitForTypeAssociatedTypeSynNodeDecl,
    ) -> Self {
        TraitForTypeAssociatedTypeSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path,
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::True,
                AllowSelfValue::False,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedTypeSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitForTypeAssociatedTypeSynDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssociatedTypeSynDecl,
    ) -> Self {
        let TraitForTypeItemSynNodeDefn::AssociatedType(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        TraitForTypeAssociatedTypeSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body_with_syn_expr_region(db),
        )
    }
}
