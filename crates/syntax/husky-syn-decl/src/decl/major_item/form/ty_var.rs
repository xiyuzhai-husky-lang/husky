use super::*;

#[salsa::tracked]
pub struct TypeVarSynNodeDecl {
    #[id]
    pub syn_node_path: FormSynNodePath,
    // todo: trais
    #[return_ref]
    pub eq_token: SynNodeDeclResult<Option<EqRegionalToken>>,
    pub default: Option<SynExprIdx>,
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
        let eq_token = parser.try_parse_option().map_err(Into::into);
        let default = if let Ok(Some(_)) = eq_token {
            parser.parse_expr_root(None, SynExprRootKind::TypeVarDefault)
        } else {
            None
        };
        TypeVarSynNodeDecl::new(self.db(), syn_node_path, eq_token, default, parser.finish())
    }
}

#[salsa::tracked]
pub struct TypeVarSynDecl {
    #[id]
    pub path: MajorFormPath,
    pub default: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeVarSynDecl {
    pub(super) fn from_node(
        db: &::salsa::Db,
        path: MajorFormPath,
        syn_node_decl: TypeVarSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        let default = syn_node_decl.default(db);
        Ok(Self::new(db, path, default, syn_expr_region))
    }
}
