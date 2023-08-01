use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedFnSynNodeDecl {
    #[id]
    pub path: TraitItemPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub template_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    #[return_ref]
    pub parenate_parameter_decl_list: NodeDeclResult<SelfParameterAndExplicitParameters<false>>,
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeExprBeforeColon>>,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitAssociatedFnSynNodeDecl {
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
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitAssociatedFnSynDecl {
    #[id]
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub syn_expr_region: SynExprRegion,
}

impl<'a> DeclParser<'a> {}
