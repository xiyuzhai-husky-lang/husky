use husky_print_utils::p;

use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeMethodFnSynNodeDecl {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub ritchie_parameter_decl_list: SynNodeDeclResult<RitchieParameters<true>>,
    pub light_arrow_token: TokenDataResult<Option<LightArrowRegionalToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeColonObelisk>>,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMethodFnSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(
                    self.ritchie_parameter_decl_list(db)
                        .as_ref()
                        .err()
                        .into_iter(),
                )
                .chain(self.return_ty(db).as_ref().err().into_iter())
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> DeclParserFactory<'a, TypeItemSynNodePath> {
    pub(super) fn parse_ty_method_node_decl(&self) -> TypeMethodFnSynNodeDecl {
        let db = self.db();
        let impl_block_syn_node_decl = self.syn_node_path().impl_block(db).syn_node_decl(db);
        let mut parser = self.parser(
            Some(impl_block_syn_node_decl.syn_expr_region(db)),
            AllowSelfType::True,
            AllowSelfValue::True,
            None,
        );
        let template_parameter_decl_list = parser.try_parse_option();
        let parameter_decl_list =
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
        TypeMethodFnSynNodeDecl::new(
            db,
            self.syn_node_path(),
            template_parameter_decl_list,
            parameter_decl_list,
            light_arrow_token,
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
    pub self_value_parameter: Option<SelfParameterObelisk>,
    #[return_ref]
    pub parenate_parameters: ExplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeBeforeColonObelisk>,
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
        let parenate_parameter_decl_list =
            syn_node_decl.ritchie_parameter_decl_list(db).as_ref()?;
        let self_value_parameter = *parenate_parameter_decl_list.self_value_parameter();
        let parenate_parameters: ExplicitParameterDeclPatterns = parenate_parameter_decl_list
            .parenate_parameters()
            .iter()
            .map(Clone::clone)
            .collect();
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(TypeMethodFnSynDecl::new(
            db,
            path,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
            syn_expr_region,
        ))
    }

    pub fn impl_block_path(self, db: &dyn SynDeclDb) -> TypeImplBlockPath {
        self.path(db).impl_block(db)
    }
}
