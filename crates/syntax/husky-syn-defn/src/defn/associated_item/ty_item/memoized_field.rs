use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeMemoizedFieldSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMemoizedFieldSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TypeItemSynNodePath,
        syn_node_decl: TypeMemoizedFieldSynNodeDecl,
    ) -> TypeMemoizedFieldSynNodeDefn {
        let mut parser = stmt_context(
            db,
            syn_node_path,
            syn_node_decl.syn_expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TypeMemoizedFieldSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            body,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMemoizedFieldSynDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMemoizedFieldSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeMemoizedFieldSynDecl,
    ) -> DeclResult<TypeMemoizedFieldSynDefn> {
        let TypeItemSynNodeDefn::MemoizedField(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        Ok(TypeMemoizedFieldSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.syn_expr_region(db),
        ))
    }
}
