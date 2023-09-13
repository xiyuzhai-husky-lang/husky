use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct StructureTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    pub syn_expr_region: SynExprRegion,
}

impl StructureTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

impl<'a> DeclParser<'a, TypeSynNodePath> {
    pub(super) fn parse_structure_ty_node_decl(&self) -> TypeSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::True, None);
        let template_parameters = parser.try_parse_option();
        StructureTypeSynNodeDecl::new(
            self.db(),
            self.syn_node_path(),
            template_parameters,
            parser.finish(),
        )
        .into()
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct StructureTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl StructureTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: StructureTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(StructureTypeSynDecl::new(
            db,
            path,
            template_parameters,
            syn_expr_region,
        ))
    }
}
