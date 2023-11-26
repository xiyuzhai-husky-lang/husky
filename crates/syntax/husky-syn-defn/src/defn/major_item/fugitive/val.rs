use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct ValSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_node_decl: ValFugitiveSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl ValSynNodeDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        syn_node_path: FugitiveSynNodePath,
        syn_node_decl: ValFugitiveSynNodeDecl,
    ) -> Self {
        ValSynNodeDefn::new_inner(
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
pub struct ValSynDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: ValFugitiveSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl ValSynDefn {
    pub(super) fn new(db: &::salsa::Db, path: FugitivePath, decl: ValFugitiveSynDecl) -> Self {
        let FugitiveSynNodeDefn::Val(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        Self::new_inner(db, path, decl, syn_node_defn.body_with_syn_expr_region(db))
    }
}
