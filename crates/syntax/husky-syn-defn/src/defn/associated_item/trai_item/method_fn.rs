use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node_decl: TraitMethodFnSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMethodFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TraitItemSynNodePath,
        syn_node_decl: TraitMethodFnSynNodeDecl,
    ) -> Self {
        let mut parser = {
            let decl_expr_region = syn_node_decl.syn_expr_region(db);
            let allow_self_type = AllowSelfType::True;
            let allow_self_value = AllowSelfValue::True;
            SynStmtContext::new(
                syn_node_path,
                decl_expr_region,
                allow_self_type,
                allow_self_value,
                db,
            )
        };
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        Self::new_inner(db, syn_node_path, syn_node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitMethodFnSynDefn {
    #[id]
    pub path: TraitItemPath,
    pub decl: TraitMethodFnSynDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMethodFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitItemPath,
        decl: TraitMethodFnSynDecl,
    ) -> SynDefnResult<Self> {
        let TraitItemSynNodeDefn::MethodFn(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        Ok(TraitMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.syn_expr_region(db),
        ))
    }
}
