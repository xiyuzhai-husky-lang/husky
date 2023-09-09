use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeAssociatedFnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeAssociatedFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TypeItemSynNodePath,
        syn_node_decl: TypeAssociatedFnSynNodeDecl,
    ) -> Self {
        Self::new_inner(
            db,
            syn_node_path,
            syn_node_decl,
            parse_defn_block_expr(
                syn_node_path,
                syn_node_decl.syn_expr_region(db),
                AllowSelfType::True,
                AllowSelfValue::False,
                db,
            ),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeAssociatedFnSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeAssociatedFnSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeAssociatedFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeAssociatedFnSynDecl,
    ) -> DeclResult<TypeAssociatedFnSynDefn> {
        let TypeItemSynNodeDefn::AssociatedFn(syn_node_defn) =
            path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        Ok(TypeAssociatedFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body_with_syn_expr_region(db),
        ))
    }
}
