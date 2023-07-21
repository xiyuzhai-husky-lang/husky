use super::*;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedFnNodeDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub node_decl: TraitAssociatedFnNodeDecl,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedFnDefn {
    #[id]
    pub node_path: TraitItemNodePath,
    pub decl: TraitAssociatedFnDecl,
    pub expr_region: ExprRegion,
}

impl TraitAssociatedFnDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TraitItemPath,
        decl: TraitAssociatedFnDecl,
    ) -> DeclResult<TraitAssociatedFnDefn> {
        todo!()
        // let node_path = decl.node_path(db);
        // let mut parser = expr_parser(
        //     db,
        //     node_path,
        //     Some(decl.expr_region(db)),
        //     AllowSelfType::True,
        //     AllowSelfValue::True,
        // );
        // let ast_idx = decl.ast_idx(db);
        // let body = match parser.ast_sheet()[ast_idx] {
        //     Ast::Defn {
        //         block: DefnBlock::AssociatedItem { body },
        //         ..
        //     } => body.map(|body| parser.parse_block_expr(body)),
        //     _ => unreachable!(),
        // };
        // TraitForTypeMethodFnDefn::new(db, node_path, decl, body, parser.finish())
    }
}
