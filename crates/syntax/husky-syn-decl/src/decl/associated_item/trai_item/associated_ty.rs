use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub syn_node: TraitItemSynNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub generics: SynNodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub eq_token: SynNodeDeclResult<EqToken>,
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

impl<'a> DeclParserFactory<'a> {
    pub(super) fn parse_trai_associated_ty_node_decl(
        &self,
        syn_node_path: TraitItemSynNodePath,
        node: TraitItemSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitAssociatedTypeSynNodeDecl {
        let db = self.db();
        let parent_trai_syn_node_decl = syn_node_path
            .parent_trai_syn_node_path(db)
            .syn_node_decl(db);
        let mut parser = self.expr_parser(
            node.syn_node_path(db),
            Some(parent_trai_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let eq_token =
            ctx.try_parse_expected(OriginalSynNodeDeclError::ExpectedEqForAssociatedType);
        let ty_term_expr_idx = ctx.parse_expr_expected2(
            None,
            ExprRootKind::AssociatedTypeTerm,
            OriginalExprError::ExpectedTypeTermForAssociatedType,
        );
        let generics = ctx.try_parse_option();
        TraitAssociatedTypeSynNodeDecl::new(
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
pub struct TraitAssociatedTypeSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl<'a> DeclParserFactory<'a> {}
