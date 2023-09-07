use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitAssociatedTypeSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitAssociatedTypeSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TraitItemSynNodePath,
        syn_node_decl: TraitAssociatedTypeSynNodeDecl,
    ) -> Self {
        let syn_node_path = syn_node_decl.syn_node_path(db);
        let mut parser = expr_parser(
            db,
            syn_node_path,
            syn_node_decl.syn_expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TraitAssociatedTypeSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            body,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeSynDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitAssociatedTypeSynDecl,
    pub syn_expr_region: SynExprRegion,
}
