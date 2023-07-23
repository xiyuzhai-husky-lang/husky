use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct FnSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_node_decl: FnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl FnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: FugitiveSynNodePath,
        syn_node_decl: FnNodeDecl,
    ) -> Self {
        let syn_node_path = syn_node_decl.syn_node_path(db);
        let mut parser = expr_parser(
            db,
            syn_node_path,
            syn_node_decl.expr_region(db),
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::Fugitive { body, .. },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        FnSynNodeDefn::new_inner(db, syn_node_path, syn_node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct FnSynDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: FnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl FnSynDefn {
    pub(super) fn new(db: &dyn SynDefnDb, path: FugitivePath, decl: FnDecl) -> Self {
        let FugitiveSynNodeDefn::Fn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
            unreachable!()
        };
        FnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.expr_region(db),
        )
    }
}
