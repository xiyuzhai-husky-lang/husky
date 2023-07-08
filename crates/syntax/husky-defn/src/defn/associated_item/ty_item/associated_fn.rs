use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnNodeDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node_decl: TypeAssociatedFnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeAssociatedFnNodeDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        node_path: TypeItemNodePath,
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

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeAssociatedFnDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedFnDecl,
    ) -> DeclResult<TypeAssociatedFnDefn> {
        let TypeItemNodeDefn::AssociatedFn(node_defn) = path.node_path(db).node_defn(db) else {
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
