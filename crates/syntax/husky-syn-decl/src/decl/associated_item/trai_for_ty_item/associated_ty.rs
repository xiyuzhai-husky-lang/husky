use husky_print_utils::p;

use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    #[return_ref]
    pub generics: SynNodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqRegionalToken>,
    // todo: change this to NodeDeclResult??
    pub ty_term_expr_idx: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedTypeSynNodeDecl {
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

impl<'a> DeclParserFactory<'a, TraitForTypeItemSynNodePath> {
    pub(super) fn parse_trai_for_ty_associated_ty_node_decl(
        &self,
    ) -> TraitForTypeAssociatedTypeSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = self.syn_node_path().impl_block(db).syn_node_decl(db);
        let mut parser = self.parser(
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
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
        TraitForTypeAssociatedTypeSynNodeDecl::new(
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
pub struct TraitForTypeAssociatedTypeSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub ty_term_expr_idx: SynExprIdx,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedTypeSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TraitForTypeItemPath,
        syn_node_decl: TraitForTypeAssociatedTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .generics(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        let ty_term_expr_idx = syn_node_decl.ty_term_expr_idx(db);
        Ok(TraitForTypeAssociatedTypeSynDecl::new(
            db,
            path,
            template_parameters,
            ty_term_expr_idx,
            syn_expr_region,
        ))
    }
}
