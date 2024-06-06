use super::*;

#[salsa::tracked]
pub struct TypeVarSynNodeDecl {
    #[id]
    pub syn_node_path: FormSynNodePath,
    // todo: trais
    pub syn_expr_region: SynExprRegion,
}

impl TypeVarSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        // chain_as_ref_err_collect!(,)
        Default::default()
    }
}

impl<'a> ItemSynNodeDeclParser<'a> {
    pub(super) fn parse_ty_var_syn_node_decl(
        &self,
        syn_node_path: FormSynNodePath,
    ) -> TypeVarSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::False, AllowSelfValue::False, None);
        TypeVarSynNodeDecl::new(self.db(), syn_node_path, parser.finish())
    }
}

#[salsa::tracked]
pub struct TypeVarSynDecl {
    #[id]
    pub path: MajorFormPath,
    pub syn_expr_region: SynExprRegion,
}

impl TypeVarSynDecl {
    pub(super) fn from_node(
        db: &::salsa::Db,
        path: MajorFormPath,
        syn_node_decl: TypeVarSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, syn_expr_region))
    }
}
