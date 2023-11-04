use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    #[return_ref]
    pub generics: SynNodeDeclResult<Option<SynTemplateParameterObeliskList>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    pub ty_term_expr_idx: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}

impl TraitAssociatedTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.generics(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eq_token(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a, TraitItemSynNodePath> {
    pub(super) fn parse_trai_associated_ty_node_decl(&self) -> TraitAssociatedTypeSynNodeDecl {
        let db = self.db();
        let parent_trai_syn_node_decl = self
            .syn_node_path()
            .parent_trai_syn_node_path(db)
            .syn_node_decl(db);
        let mut parser = self.expr_parser(
            Some(parent_trai_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
        );
        let eq_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEqForAssociatedType);
        let ty_term_expr_idx = parser.parse_expr_expected2(
            None,
            ExprRootKind::AssociatedTypeTerm,
            OriginalSynExprError::ExpectedTypeTermForAssociatedType,
        );
        let generics = parser.try_parse_option();
        TraitAssociatedTypeSynNodeDecl::new(
            db,
            self.syn_node_path(),
            generics,
            eq_token,
            ty_term_expr_idx,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedTypeSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: SynTemplateParameterObelisks,
    pub syn_expr_region: SynExprRegion,
}
