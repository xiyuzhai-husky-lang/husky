use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct GnSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_node_decl: GnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl GnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: FugitiveSynNodePath,
        syn_node_decl: GnSynNodeDecl,
    ) -> Self {
        GnSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path,
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::False,
                AllowSelfValue::False,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct GnSynDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: FunctionGnSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl GnSynDefn {
    pub(super) fn new(db: &dyn SynDefnDb, path: FugitivePath, decl: FunctionGnSynDecl) -> Self {
        let FugitiveSynNodeDefn::Gn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        GnSynDefn::new_inner(db, path, decl, syn_node_defn.body_with_syn_expr_region(db))
    }
}
