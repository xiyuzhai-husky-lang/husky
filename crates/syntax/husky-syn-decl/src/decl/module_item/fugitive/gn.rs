use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct GnSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub ast_idx: AstIdx,
    pub syn_expr_region: SynExprRegion,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    #[return_ref]
    parenate_parameter_decl_list: SynNodeDeclResult<RitchieParameters<false>>,
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeColonObelisk>>,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolToken>,
}

impl GnSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
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

impl<'a> DeclParserFactory<'a> {
    pub(super) fn parse_gn_node_decl(
        &self,
        syn_node_path: FugitiveSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> GnSynNodeDecl {
        let mut parser = self.parser(
            syn_node_path,
            None,
            AllowSelfType::False,
            AllowSelfValue::False,
            None,
            token_group_idx,
            Some(saved_stream_state),
        );
        let template_parameter_decl_list = parser.try_parse_option();
        let parameter_decl_list =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedParameterDeclList);

        let curry_token = parser.try_parse_option();
        let return_ty = if let Ok(Some(_)) = curry_token {
            parser
                .try_parse_expected(OriginalSynNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEolColon);
        GnSynNodeDecl::new(
            self.db(),
            syn_node_path,
            ast_idx,
            parser.finish(),
            template_parameter_decl_list,
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
    pub template_parameters: ImplicitParameterDeclPatterns,
    #[return_ref]
    pub parenate_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeBeforeColonObelisk>,
    pub syn_expr_region: SynExprRegion,
}

impl GnSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: FugitivePath,
        syn_node_decl: GnSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let parenate_parameter_decl_list =
            syn_node_decl.parenate_parameter_decl_list(db).as_ref()?;
        let parenate_parameters: ExplicitParameterDeclPatterns = parenate_parameter_decl_list
            .parenate_parameters()
            .iter()
            .map(Clone::clone)
            .collect();
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(GnSynDecl::new(
            db,
            path,
            template_parameters,
            parenate_parameters,
            return_ty,
            syn_expr_region,
        ))
    }
}
