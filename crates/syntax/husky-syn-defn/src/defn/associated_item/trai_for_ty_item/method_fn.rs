use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeMethodFnSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeMethodFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TraitForTypeItemSynNodePath,
        syn_node_decl: TraitForTypeMethodFnSynNodeDecl,
    ) -> Self {
        let mut ctx = SynStmtContext::new(
            syn_node_path,
            syn_node_decl.syn_expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::True,
            db,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match ctx.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| ctx.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TraitForTypeMethodFnSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            body,
            ctx.finish(),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeMethodFnSynDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeMethodFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeMethodFnSynDecl,
    ) -> TraitForTypeMethodFnSynDefn {
        let TraitForTypeItemSynNodeDefn::MethodFn(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        TraitForTypeMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.syn_expr_region(db),
        )
    }
}
