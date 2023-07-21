use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub node_decl: TypeMemoizedFieldNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TypeMemoizedFieldNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TypeItemSynNodePath,
        node_decl: TypeMemoizedFieldNodeDecl,
    ) -> TypeMemoizedFieldNodeDefn {
        let mut parser = expr_parser(
            db,
            syn_node_path,
            node_decl.expr_region(db),
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
        TypeMemoizedFieldNodeDefn::new_inner(db, syn_node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMemoizedFieldDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TypeMemoizedFieldDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeMemoizedFieldDecl,
    ) -> DeclResult<TypeMemoizedFieldDefn> {
        let TypeItemSynNodeDefn::MemoizedField(node_defn) = path.syn_node_path(db).node_defn(db) else {
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
