use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct GnSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    #[return_ref]
    parenic_parameter_decl_list: NodeDeclResult<SelfParameterAndExplicitParameters<false>>,
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeExprBeforeColon>>,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
}

impl GnSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
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
    pub(super) fn parse_gn_node_decl(
        &self,
        syn_node_path: FugitiveSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> GnSynNodeDecl {
        let mut parser = self.expr_parser(
            syn_node_path,
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameter_decl_list = ctx.try_parse_option();
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
        GnSynNodeDecl::new(
            self.db(),
            syn_node_path,
            ast_idx,
            parser.finish(),
            implicit_parameter_decl_list,
            parameter_decl_list,
            curry_token,
            return_ty,
            eol_colon,
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct GnSynDecl {
    #[id]
    pub path: FugitivePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub parenic_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub syn_expr_region: SynExprRegion,
}

impl GnSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: FugitivePath,
        syn_node_decl: GnSynNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = syn_node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let parenic_parameter_decl_list = syn_node_decl.parenic_parameter_decl_list(db).as_ref()?;
        let parenic_parameters: ExplicitParameterDeclPatterns = parenic_parameter_decl_list
            .parenic_parameters()
            .iter()
            .map(Clone::clone)
            .collect();
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(GnSynDecl::new(
            db,
            path,
            generic_parameters,
            parenic_parameters,
            return_ty,
            syn_expr_region,
        ))
    }
}
