use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct AlienTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
}

impl AlienTypeDecl {
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
