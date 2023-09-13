use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnSynNodeDecl {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    #[return_ref]
    pub template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub parenate_parameter_decl_list: SynNodeDeclResult<RitchieParameters<true>>,
    #[return_ref]
    pub light_arrow_token: TokenDataResult<Option<LightArrowRegionalToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeColonObelisk>>,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl From<TraitMethodFnSynNodeDecl> for ItemSynNodeDecl {
    fn from(syn_node_decl: TraitMethodFnSynNodeDecl) -> Self {
        todo!()
    }
}

impl TraitMethodFnSynNodeDecl {
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

impl<'a> DeclParserFactory<'a, TraitItemSynNodePath> {
    pub(super) fn parse_trai_method_fn_node_decl(&self) -> TraitMethodFnSynNodeDecl {
        let db = self.db();
        let parent_trai_syn_node_decl = self
            .syn_node_path()
            .parent_trai_syn_node_path(db)
            .syn_node_decl(db);
        let mut parser = self.parser(
            Some(parent_trai_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
            None,
        );
        let template_parameter_decl_list = parser.try_parse_option();
        let parenate_parameter_decl_list =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedParameterDeclList);
        let light_arrow_token = parser.try_parse_option();
        let return_ty = if let Ok(Some(_)) = light_arrow_token {
            parser
                .try_parse_expected(OriginalSynNodeDeclError::ExpectedOutputType)
                .map(Some)
        } else {
            Ok(None)
        };
        let eol_colon = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEolColon);
        TraitMethodFnSynNodeDecl::new(
            db,
            self.syn_node_path(),
            template_parameter_decl_list,
            parenate_parameter_decl_list,
            light_arrow_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitMethodFnSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub self_value_parameter: Option<SelfParameterObelisk>,
    #[return_ref]
    pub parenate_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeBeforeColonObelisk>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitMethodFnSynDecl {}
