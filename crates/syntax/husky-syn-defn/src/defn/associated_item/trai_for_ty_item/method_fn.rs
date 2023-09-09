use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeMethodFnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitForTypeMethodFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TraitForTypeItemSynNodePath,
        syn_node_decl: TraitForTypeMethodFnSynNodeDecl,
    ) -> Self {
        TraitForTypeMethodFnSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path,
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::True,
                AllowSelfValue::True,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeMethodFnSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitForTypeMethodFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeMethodFnSynDecl,
    ) -> TraitForTypeMethodFnSynDefn {
        let TraitForTypeItemSynNodeDefn::MethodFn(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        TraitForTypeMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body_with_syn_expr_region(db),
        )
    }
}
