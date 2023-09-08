use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeAssociatedTypeSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedTypeSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TraitForTypeItemSynNodePath,
        syn_node_decl: TraitForTypeAssociatedTypeSynNodeDecl,
    ) -> Self {
        let syn_node_path = syn_node_decl.syn_node_path(db);
        let mut ctx = stmt_context(
            db,
            syn_node_path,
            syn_node_decl.syn_expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match ctx.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| ctx.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TraitForTypeAssociatedTypeSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            body,
            ctx.finish(),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedTypeSynDecl,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedTypeSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssociatedTypeSynDecl,
    ) -> Self {
        let TraitForTypeItemSynNodeDefn::AssociatedType(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        TraitForTypeAssociatedTypeSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.syn_expr_region(db),
        )
    }
}
