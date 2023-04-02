use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct InductiveTypeDecl {
    #[id]
    pub path: TypePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl InductiveTypeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_inductive_ty_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        variants: TypeVariants,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<TypeDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.parse()?;
        Ok(InductiveTypeDecl::new(
            self.db(),
            path,
            ast_idx,
            parser.finish(),
            implicit_parameters,
        )
        .into())
    }
}
