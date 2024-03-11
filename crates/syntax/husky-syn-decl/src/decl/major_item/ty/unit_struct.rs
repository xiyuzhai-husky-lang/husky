use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnitStructSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
}

impl UnitStructSynNodeDecl {
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
pub struct UnitStructSynDecl {
    #[id]
    pub path: TypePath,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
}

impl UnitStructSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        _db: &::salsa::Db,
        _path: TypePath,
        _syn_node_decl: UnitStructSynNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
