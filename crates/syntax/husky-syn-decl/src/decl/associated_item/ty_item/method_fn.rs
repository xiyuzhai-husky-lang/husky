use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeMethodFnSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub node: TypeItemSynNode,
    pub ast_idx: AstIdx,
    #[return_ref]
    template_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub parenic_parameter_decl_list: NodeDeclResult<SelfParameterAndExplicitParameters<true>>,
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeExprBeforeColon>>,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMethodFnSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(
                    self.parenic_parameter_decl_list(db)
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
    pub(super) fn parse_ty_method_node_decl(
        &self,
        syn_node_path: TypeItemSynNodePath,
        node: TypeItemSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TypeMethodFnSynNodeDecl {
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
        let parameter_decl_list =
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedParameterDeclList);
        let curry_token = ctx.try_parse_option();
        let return_ty = if let Ok(Some(_)) = curry_token {
            ctx.try_parse_expected(OriginalNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = ctx.try_parse_expected(OriginalNodeDeclError::ExpectedEolColon);
        TypeMethodFnSynNodeDecl::new(
            db,
            syn_node_path,
            node,
            ast_idx,
            template_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct TypeMethodFnSynDecl {
    #[id]
    pub path: TypeItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub self_parameter: Option<SelfParameterDeclPattern>,
    #[return_ref]
    pub parenic_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMethodFnSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeItemPath,
        syn_node_decl: TypeMethodFnSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let parenic_parameter_decl_list = syn_node_decl.parenic_parameter_decl_list(db).as_ref()?;
        let self_parameter = *parenic_parameter_decl_list.self_parameter();
        let parenic_parameters: ExplicitParameterDeclPatterns = parenic_parameter_decl_list
            .parenic_parameters()
            .iter()
            .map(Clone::clone)
            .collect();
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(TypeMethodFnSynDecl::new(
            db,
            path,
            template_parameters,
            self_parameter,
            parenic_parameters,
            return_ty,
            syn_expr_region,
        ))
    }

    pub fn impl_block_path(self, db: &dyn SynDeclDb) -> TypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
