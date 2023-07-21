use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedFnNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub node_decl: TraitAssociatedFnNodeDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedFnDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub decl: TraitAssociatedFnDecl,
    pub expr_region: SynExprRegion,
}

impl TraitAssociatedFnDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitItemPath,
        decl: TraitAssociatedFnDecl,
    ) -> DeclResult<TraitAssociatedFnDefn> {
        todo!()
        // let syn_node_path = decl.syn_node_path(db);
        // let mut parser = expr_parser(
        //     db,
        //     syn_node_path,
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
        // TraitForTypeMethodFnDefn::new(db, syn_node_path, decl, body, parser.finish())
    }
}
