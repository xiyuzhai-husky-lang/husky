use super::*;

#[salsa::tracked]
pub struct TraitAssocTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    #[return_ref]
    pub template_parameters: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub syn_expr_region: SynExprRegion,
}

/// # getters
impl TraitAssocTypeSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(self.template_parameters(db).as_ref().err().into_iter())
    }
}

/// # parse
impl<'a> ItemSynNodeDeclParser<'a> {
    pub(super) fn parse_trai_assoc_ty_node_decl(
        &self,
        syn_node_path: TraitItemSynNodePath,
    ) -> TraitAssocTypeSynNodeDecl {
        let db = self.db();
        let parent_trai_syn_node_decl = syn_node_path
            .data(db)
            .parent_trai_syn_node_path
            .syn_node_decl(db);
        let mut parser = self.expr_parser(
            Some(parent_trai_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
        );
        let template_parameters = parser.try_parse_option();
        TraitAssocTypeSynNodeDecl::new(db, syn_node_path, template_parameters, parser.finish())
    }
}

#[salsa::tracked]
pub struct TraitAssocTypeSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub syn_expr_region: SynExprRegion,
}

impl TraitAssocTypeSynDecl {
    pub(super) fn from_node(
        path: TraitItemPath,
        syn_node_decl: TraitAssocTypeSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameters(db)
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
