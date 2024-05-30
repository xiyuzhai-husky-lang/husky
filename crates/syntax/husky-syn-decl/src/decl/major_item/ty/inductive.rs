use super::*;

#[salsa::tracked]
pub struct InductiveSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub syn_expr_region: SynExprRegion,
}

impl InductiveSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

impl<'a> ItemDeclParser<'a> {
    pub(super) fn parse_inductive_ty_node_decl(
        &self,
        syn_node_path: TypeSynNodePath,
    ) -> InductiveSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        let template_parameter_decl_list = parser.try_parse_option();
        InductiveSynNodeDecl::new(
            self.db(),
            syn_node_path,
            template_parameter_decl_list,
            parser.finish(),
        )
    }
}

#[salsa::tracked]
pub struct InductiveSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub syn_expr_region: SynExprRegion,
}

impl InductiveSynDecl {
    #[inline(always)]
    pub(super) fn from_node(
        db: &::salsa::Db,
        path: TypePath,
        syn_node_decl: InductiveSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, template_parameters, syn_expr_region))
    }
}
