use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnitStructTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
}

impl UnitStructTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnitStructTypeSynDecl {
    #[id]
    pub path: TypePath,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
}

impl UnitStructTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: UnitStructTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
