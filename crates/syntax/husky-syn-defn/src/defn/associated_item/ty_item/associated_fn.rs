use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnNodeDefn {
    #[id]
    pub node_path: TypeItemSynNodePath,
    pub node_decl: TypeAssociatedFnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedFnNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        node_path: TypeItemSynNodePath,
        node_decl: TypeAssociatedFnNodeDecl,
    ) -> Self {
        let mut parser = expr_parser(
            db,
            node_path,
            node_decl.expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let ast_idx = node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TypeAssociatedFnNodeDefn::new_inner(db, node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TypeAssociatedFnDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedFnDecl,
    ) -> DeclResult<TypeAssociatedFnDefn> {
        let TypeItemSynNodeDefn::AssociatedFn(node_defn) = path.syn_node_path(db).node_defn(db) else {
            unreachable!()
        };
        Ok(TypeAssociatedFnDefn::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        ))
    }
}
