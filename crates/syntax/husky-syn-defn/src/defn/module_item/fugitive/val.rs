use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct ValNodeDefn {
    #[id]
    pub node_path: FugitiveNodePath,
    pub node_decl: ValNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl ValNodeDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        node_path: FugitiveNodePath,
        node_decl: ValNodeDecl,
    ) -> Self {
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
        ValNodeDefn::new_inner(db, node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct ValDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: ValDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl ValDefn {
    pub(super) fn new(db: &dyn DefnDb, path: FugitivePath, decl: ValDecl) -> Self {
        let FugitiveNodeDefn::Val(node_defn) = path.node_path(db).node_defn(db) else {
            unreachable!()
        };
        Self::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        )
    }
}
