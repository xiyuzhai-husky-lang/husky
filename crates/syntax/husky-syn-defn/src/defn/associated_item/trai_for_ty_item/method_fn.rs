use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node_decl: TraitForTypeMethodFnSynNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TraitForTypeMethodFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TraitForTypeItemSynNodePath,
        syn_node_decl: TraitForTypeMethodFnSynNodeDecl,
    ) -> Self {
        let mut parser = expr_parser(
            db,
            syn_node_path,
            syn_node_decl.expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TraitForTypeMethodFnSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            body,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnSynDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeMethodFnSynDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TraitForTypeMethodFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeMethodFnSynDecl,
    ) -> TraitForTypeMethodFnSynDefn {
        let TraitForTypeItemSynNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
            unreachable!()
        };
        TraitForTypeMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.expr_region(db),
        )
    }
}
