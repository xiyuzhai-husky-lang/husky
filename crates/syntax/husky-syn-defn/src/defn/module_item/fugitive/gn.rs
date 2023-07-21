use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct GnNodeDefn {
    #[id]
    pub node_path: FugitiveSynNodePath,
    pub node_decl: GnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl GnNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        node_path: FugitiveSynNodePath,
        node_decl: GnNodeDecl,
    ) -> Self {
        let node_path = node_decl.node_path(db);
        let mut parser = expr_parser(
            db,
            node_path,
            node_decl.expr_region(db),
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let ast_idx = node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::Fugitive { body, .. },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        GnNodeDefn::new_inner(db, node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct GnDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: GnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl GnDefn {
    pub(super) fn new(db: &dyn SynDefnDb, path: FugitivePath, decl: GnDecl) -> Self {
        let FugitiveSynNodeDefn::Gn(node_defn) = path.node_path(db).node_defn(db) else {
            unreachable!()
        };
        GnDefn::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        )
    }
}
