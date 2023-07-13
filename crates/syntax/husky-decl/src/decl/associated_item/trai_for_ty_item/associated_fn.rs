use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedFnNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub implicit_parameter_decl_list: NodeDeclResult<Option<ImplicitParameterDeclList>>,
    #[return_ref]
    pub explicit_parameter_decl_list: NodeDeclResult<SelfParameterAndExplicitParameters<false>>,
    pub curry_token: TokenResult<Option<CurryToken>>,
    #[return_ref]
    pub return_ty: NodeDeclResult<Option<ReturnTypeExprBeforeColon>>,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeAssociatedFnNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(
                    self.explicit_parameter_decl_list(db)
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

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitForTypeAssociatedFnDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub return_ty: Option<ReturnTypeExprBeforeColon>,
    pub expr_region: ExprRegion,
}

impl TraitForTypeAssociatedFnDecl {
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitForTypeItemPath,
        node_decl: TraitForTypeAssociatedFnNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
