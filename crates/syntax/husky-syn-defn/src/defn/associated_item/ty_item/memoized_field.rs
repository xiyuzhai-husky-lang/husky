use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeMemoizedFieldSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeMemoizedFieldSynNodeDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        syn_node_path: TypeItemSynNodePath,
        syn_node_decl: TypeMemoizedFieldSynNodeDecl,
    ) -> TypeMemoizedFieldSynNodeDefn {
        TypeMemoizedFieldSynNodeDefn::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path,
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::True,
                AllowSelfValue::True,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMemoizedFieldSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMemoizedFieldSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeMemoizedFieldSynDefn {
    pub(super) fn new(
        db: &::salsa::Db,
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
            syn_node_defn.body_with_syn_expr_region(db),
        ))
    }
}
