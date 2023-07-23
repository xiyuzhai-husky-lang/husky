use super::*;

impl TypeAssociatedFnHirNodeDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        syn_node_path: TypeItemHirNodePath,
        syn_node_decl: TypeAssociatedFnHirNodeDecl,
    ) -> Self {
        let mut parser = expr_parser(
            db,
            syn_node_path,
            syn_node_decl.expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TypeAssociatedFnHirNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            body,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnHirDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedFnHirDecl,
    pub body: Option<HirExprIdx>,
    pub expr_region: HirExprRegion,
}

impl TypeAssociatedFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedFnHirDecl,
    ) -> DeclResult<TypeAssociatedFnHirDefn> {
        let TypeItemHirNodeDefn::AssociatedFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
            unreachable!()
        };
        Ok(TypeAssociatedFnHirDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.expr_region(db),
        ))
    }
}
