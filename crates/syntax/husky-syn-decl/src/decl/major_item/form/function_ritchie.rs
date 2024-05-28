use super::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked]
pub struct MajorFunctionRitchieSynNodeDecl {
    #[id]
    pub syn_node_path: FormSynNodePath,
    /// it can be derived equally from syn_node_path
    pub ritchie_item_kind: RitchieItemKind,
    #[return_ref]
    pub template_parameter_obelisk_list:
        SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    #[return_ref]
    parenate_parameter_obelisk_list: SynNodeDeclResult<ParenateParameterSyndicateList<false>>,
    #[return_ref]
    pub light_arrow_token: TokenDataResult<Option<LightArrowRegionalToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeColonSyndicate>>,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

/// # getters
impl MajorFunctionRitchieSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_obelisk_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(
                    self.parenate_parameter_obelisk_list(db)
                        .as_ref()
                        .err()
                        .into_iter(),
                )
                .chain(self.return_ty(db).as_ref().err().into_iter())
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

impl<'a> ItemDeclParser<'a> {
    pub(super) fn parse_ritchie_syn_node_decl(
        &self,
        syn_node_path: FormSynNodePath,
        ritchie_item_kind: RitchieItemKind,
    ) -> MajorFunctionRitchieSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::False, AllowSelfValue::False, None);
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
        MajorFunctionRitchieSynNodeDecl::new(
            self.db(),
            syn_node_path,
            ritchie_item_kind,
            template_parameter_decl_list,
            parameter_decl_list,
            light_arrow_token,
            return_ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked]
pub struct MajorFunctionRitchieSynDecl {
    #[id]
    pub path: MajorFormPath,
    /// it can be derived equally from path
    pub ritchie_item_kind: RitchieItemKind,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    #[return_ref]
    pub parenate_parameters: ParenateSynParametersData,
    pub return_ty: Option<ReturnTypeBeforeColonSyndicate>,
    pub syn_expr_region: SynExprRegion,
}

impl MajorFunctionRitchieSynDecl {
    pub(super) fn from_node_decl(
        db: &::salsa::Db,
        path: MajorFormPath,
        syn_node_decl: MajorFunctionRitchieSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let ritchie_item_kind = syn_node_decl.ritchie_item_kind(db);
        debug_assert_eq!(path.major_form_kind(db), ritchie_item_kind.into());
        let template_parameters = syn_node_decl
            .template_parameter_obelisk_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let parenate_parameter_decl_list =
            syn_node_decl.parenate_parameter_obelisk_list(db).as_ref()?;
        let parenate_parameters: ParenateSynParametersData = parenate_parameter_decl_list
            .parenate_parameters()
            .iter()
            .map(Clone::clone)
            .collect();
        let return_ty = *syn_node_decl.return_ty(db).as_ref()?;
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(MajorFunctionRitchieSynDecl::new(
            db,
            path,
            ritchie_item_kind,
            template_parameters,
            parenate_parameters,
            return_ty,
            syn_expr_region,
        ))
    }
}
