use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitAssociatedFnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitAssociatedFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TraitItemSynNodePath,
        syn_node_decl: TraitAssociatedFnSynNodeDecl,
    ) -> Self {
        Self::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path,
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::True,
                AllowSelfValue::False,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedFnSynDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub decl: TraitAssociatedFnSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TraitAssociatedFnSynDefn {
    pub(super) fn new(
        _db: &dyn SynDefnDb,
        _path: TraitItemPath,
        _decl: TraitAssociatedFnSynDecl,
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
