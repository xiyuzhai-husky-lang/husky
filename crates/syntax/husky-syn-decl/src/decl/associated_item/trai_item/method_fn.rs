use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnSynNodeDecl {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub node: TraitItemSynNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub template_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub parenate_parameter_decl_list: NodeDeclResult<RitchieParameters<true>>,
    #[return_ref]
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeExprBeforeColon>>,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMethodFnSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(
                    self.parenate_parameter_decl_list(db)
                        .as_ref()
                        .err()
                        .into_iter(),
                )
                .chain(self.return_ty(db).as_ref().err().into_iter())
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_trai_method_fn_node_decl(
        &self,
        syn_node_path: TraitItemSynNodePath,
        node: TraitItemSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitMethodFnSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = syn_node_path.impl_block(db).syn_node_decl(db);
        let mut parser = self.expr_parser(
            node.syn_node_path(db),
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, saved_stream_state);
        let template_parameter_decl_list = ctx.try_parse_option();
        let parenate_parameter_decl_list =
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedParameterDeclList);
        let curry_token = ctx.try_parse_option();
        let return_ty = if let Ok(Some(_)) = curry_token {
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = ctx.try_parse_expected(OriginalNodeDeclError::ExpectedEolColon);
        TraitMethodFnSynNodeDecl::new(
            db,
            syn_node_path,
            node,
            ast_idx,
            template_parameter_decl_list,
            parenate_parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
    }
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub self_parameter: Option<SelfParameterDeclPattern>,
    #[return_ref]
    pub parenate_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMethodFnSynDecl {}
