use husky_print_utils::p;

use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub syn_node: TraitForTypeItemSynNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub generics: SynNodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqToken>,
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

impl<'a> DeclParserFactory<'a> {
    pub(super) fn parse_trai_for_ty_associated_ty_node_decl(
        &self,
        syn_node_path: TraitForTypeItemSynNodePath,
        node: TraitForTypeItemSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitForTypeAssociatedTypeSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = syn_node_path.impl_block(db).syn_node_decl(db);
        let mut parser = self.parser(
            node.syn_node_path(db),
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
            token_group_idx,
            saved_stream_state,
        );
        let eq_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEqForAssociatedType);
        let ty_term_expr_idx = parser.parse_expr_expected2(
            None,
            ExprRootKind::AssociatedTypeTerm,
            OriginalExprError::ExpectedTypeTermForAssociatedType,
        );
        let generics = parser.try_parse_option();
        TraitForTypeAssociatedTypeSynNodeDecl::new(
            db,
            node.syn_node_path(db),
            node,
            ast_idx,
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
