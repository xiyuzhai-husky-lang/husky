use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct GnSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_node_decl: GnSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl GnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: FugitiveSynNodePath,
        syn_node_decl: GnSynNodeDecl,
    ) -> Self {
        let syn_node_path = syn_node_decl.syn_node_path(db);
        let mut parser = expr_parser(
            db,
            syn_node_path,
            syn_node_decl.syn_expr_region(db),
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                block: DefnBlock::Fugitive { body, .. },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        GnSynNodeDefn::new_inner(db, syn_node_path, syn_node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct GnSynDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: GnSynDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl GnSynDefn {
    pub(super) fn new(db: &dyn SynDefnDb, path: FugitivePath, decl: GnSynDecl) -> Self {
        let FugitiveSynNodeDefn::Gn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        GnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.syn_expr_region(db),
        )
    }
}
