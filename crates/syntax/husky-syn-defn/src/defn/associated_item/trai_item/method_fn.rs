use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitMethodFnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitMethodFnSynNodeDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        syn_node_path: TraitItemSynNodePath,
        syn_node_decl: TraitMethodFnSynNodeDecl,
    ) -> Self {
        Self::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path.into(),
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::True,
                AllowSelfValue::True,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitMethodFnSynDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitMethodFnSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitMethodFnSynDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: TraitItemPath,
        decl: TraitMethodFnSynDecl,
    ) -> SynDefnResult<Self> {
        let TraitItemSynNodeDefn::MethodFn(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        Ok(TraitMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body_with_syn_expr_region(db),
        ))
    }
}
