use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssocTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    #[return_ref]
    pub template_parameters: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    pub ty_term_expr_idx: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}

/// # getters
impl TraitForTypeAssocTypeSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameters(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eq_token(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_trai_for_ty_associated_ty_node_decl(
        &self,
        syn_node_path: TraitForTypeItemSynNodePath,
    ) -> TraitForTypeAssocTypeSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = syn_node_path.data(db).impl_block(db).syn_node_decl(db);
        let mut parser = self.expr_parser(
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
        );
        let eq_token = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEqForAssocType);
        let ty_term_expr_idx = parser.parse_expr_expected2(
            None,
            SynExprRootKind::AssocTypeTerm,
            OriginalSynExprError::ExpectedTypeTermForAssocType,
        );
        let generics = parser.try_parse_option();
        TraitForTypeAssocTypeSynNodeDecl::new(
            db,
            syn_node_path,
            generics,
            eq_token,
            ty_term_expr_idx,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssocTypeSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub ty_term_expr_idx: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssocTypeSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        syn_node_decl: TraitForTypeAssocTypeSynNodeDecl,
    ) -> DeclResult<Self> {
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
        let ty_term_expr_idx = syn_node_decl.ty_term_expr_idx(db);
        Ok(TraitForTypeAssocTypeSynDecl::new(
            db,
            path,
            template_parameters,
            ty_term_expr_idx,
            syn_expr_region,
        ))
    }
}
