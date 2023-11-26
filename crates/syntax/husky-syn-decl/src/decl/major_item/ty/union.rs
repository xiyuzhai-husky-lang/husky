use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnionTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub syn_expr_region: SynExprRegion,
}

impl UnionTypeSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnionTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub syn_expr_region: SynExprRegion,
}

impl UnionTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        _db: &::salsa::Db,
        _path: TypePath,
        _syn_node_decl: UnionTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
