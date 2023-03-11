use super::*;
use husky_token::EolColonToken;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeImplDecl {
    pub ast_idx: AstIdx,
    pub impl_block: ImplBlock,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub ty: TypeExpr,
    #[return_ref]
    pub eol_colon: DeclExprResult<EolColonToken>,
    pub expr_region: ExprRegion,
}

impl TypeImplDecl {
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        self.implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(Ok(&[]))
    }
}
