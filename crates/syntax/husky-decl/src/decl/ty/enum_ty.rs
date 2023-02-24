use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct EnumTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
}

impl EnumTypeDecl {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclDb,
    ) -> DeclExprResultRef<&[ImplicitParameterDecl]> {
        self.implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(Ok(&[]))
    }
}
