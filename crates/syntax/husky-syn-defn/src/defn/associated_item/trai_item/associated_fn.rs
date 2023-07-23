use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitAssociatedFnSynNodeDecl,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedFnSynDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub decl: TraitAssociatedFnSynDecl,
    pub syn_expr_region: SynExprRegion,
}

impl TraitAssociatedFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitItemPath,
        decl: TraitAssociatedFnSynDecl,
    ) -> DeclResult<TraitAssociatedFnSynDefn> {
        todo!()
        // let syn_node_path = decl.syn_node_path(db);
        // let mut parser = expr_parser(
        //     db,
        //     syn_node_path,
        //     Some(decl.syn_expr_region(db)),
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
