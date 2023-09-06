use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedFnSynNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub template_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub parenate_parameter_decl_list: NodeDeclResult<RitchieParameters<false>>,
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeBeforeColonObelisk>>,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
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

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedFnSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
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
