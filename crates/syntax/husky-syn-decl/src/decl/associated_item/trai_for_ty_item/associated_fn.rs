use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedFnSynNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameter_decl_list: SynNodeDeclResult<Option<TemplateParameters>>,
    #[return_ref]
    pub parenate_parameter_decl_list: SynNodeDeclResult<ParenateParameters<false>>,
    pub light_arrow_token: TokenDataResult<Option<LightArrowRegionalToken>>,
    #[return_ref]
    pub return_ty: SynNodeDeclResult<Option<ReturnTypeBeforeColonObelisk>>,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedFnSynNodeDecl {
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

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedFnSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: TemplateParameterObelisks,
    pub return_ty: Option<ReturnTypeBeforeColonObelisk>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedFnSynDecl {
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TraitForTypeItemPath,
        syn_node_decl: TraitForTypeAssociatedFnSynNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
