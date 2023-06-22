use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnNodeDefn {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub node_decl: TraitForTypeMethodFnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeMethodFnNodeDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        node_path: TraitForTypeItemNodePath,
        node_decl: TraitForTypeMethodFnNodeDecl,
    ) -> Self {
        let mut parser = expr_parser(
            db,
            node_path,
            Some(node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let ast_idx = node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TraitForTypeMethodFnNodeDefn::new_inner(db, node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeMethodFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeMethodFnDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeMethodFnDecl,
    ) -> TraitForTypeMethodFnDefn {
        let TraitForTypeItemNodeDefn::MethodFn(node_defn) = path.node_path(db).node_defn(db) else {
            unreachable!()
        };
        TraitForTypeMethodFnDefn::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        )
    }
}
