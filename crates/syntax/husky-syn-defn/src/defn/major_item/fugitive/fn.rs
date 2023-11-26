use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct FnSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_node_decl: FnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl FnSynNodeDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        syn_node_path: FugitiveSynNodePath,
        syn_node_decl: FnSynNodeDecl,
    ) -> Self {
        FnSynNodeDefn::new_inner(
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
pub struct FnSynDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: FunctionFnFugitiveSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl FnSynDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: FugitivePath,
        decl: FunctionFnFugitiveSynDecl,
    ) -> Self {
        let FugitiveSynNodeDefn::FunctionFn(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        FnSynDefn::new_inner(db, path, decl, syn_node_defn.body_with_syn_expr_region(db))
    }
}
