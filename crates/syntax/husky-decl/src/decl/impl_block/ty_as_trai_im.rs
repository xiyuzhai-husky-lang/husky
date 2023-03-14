use super::*;

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn ty_as_trai_impl_decl(
    _db: &dyn DeclDb,
    _impl_block: TypeAsTraitImplBlock,
) -> DeclResult<TypeAsTraitImplDecl> {
    todo!()
    // let parser = DeclParser::new(db, impl_block.module_path(db))?;
    // Ok(parser.parse_ty_as_trai_impl_decl(impl_block)?.into())
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TypeAsTraitImplDecl {
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl TypeAsTraitImplDecl {
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
