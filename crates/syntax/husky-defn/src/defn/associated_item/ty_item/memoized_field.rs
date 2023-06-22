use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldNodeDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node_decl: TypeMemoizedFieldNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeMemoizedFieldNodeDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        node_path: TypeItemNodePath,
        node_decl: TypeMemoizedFieldNodeDecl,
    ) -> TypeMemoizedFieldNodeDefn {
        let mut parser = expr_parser(
            db,
            node_path,
            Some(node_decl.expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let ast_idx = node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TypeMemoizedFieldNodeDefn::new_inner(db, node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = DefnDb, jar = DefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMemoizedFieldDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeMemoizedFieldDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TypeItemPath,
        decl: TypeMemoizedFieldDecl,
    ) -> DeclResult<TypeMemoizedFieldDefn> {
        let TypeItemNodeDefn::MemoizedField(node_defn) = path.node_path(db).node_defn(db) else {
            unreachable!()
        };
        Ok(TypeMemoizedFieldDefn::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        ))
    }
}
