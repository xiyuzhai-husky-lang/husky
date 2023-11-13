use super::*;



#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeMethodFnSynNodeDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeMethodFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TypeItemSynNodePath,
        syn_node_decl: TypeMethodFnSynNodeDecl,
    ) -> Self {
        Self::new_inner(
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
pub struct TypeMethodFnSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMethodFnSynDecl,
    pub body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
}

impl TypeMethodFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeMethodFnSynDecl,
    ) -> SynDefnResult<Self> {
        let TypeItemSynNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        Ok(TypeMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body_with_syn_expr_region(db),
        ))
    }
}
