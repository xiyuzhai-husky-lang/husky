use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    #[return_ref]
    pub colon_token: SynNodeDeclResult<ColonRegionalToken>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<ReturnTypeBeforeEqSyndicate>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedValSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        chain_as_ref_err_collect!(self.colon_token(db), self.return_ty(db), self.eq_token(db))
    }
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAssociatedValSynDecl {
    #[id]
    pub path: TypeItemPath,
    pub return_ty: ReturnTypeBeforeEqSyndicate,
    pub expr: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeAssociatedValSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        syn_node_decl: TypeAssociatedValSynNodeDecl,
    ) -> DeclResult<Self> {
        let val_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let expr = syn_node_decl.expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(TypeAssociatedValSynDecl::new(
            db,
            path,
            val_ty,
            expr,
            syn_expr_region,
        ))
    }
}
