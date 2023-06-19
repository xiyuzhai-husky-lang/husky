use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct EnumTypeNodeDecl {
    #[id]
    pub node_id: TypeNodeId,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub expr_region: ExprRegion,
}

impl EnumTypeNodeDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct EnumTypeDecl {
    #[id]
    pub node_id: TypeNodeId,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
    pub expr_region: ExprRegion,
}

impl EnumTypeDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDeclPattern] {
        self.implicit_parameter_decl_list(db)
            .as_ref()
            .map(ImplicitParameterDeclList::implicit_parameters)
            .unwrap_or(&[])
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_enum_ty_decl(
        &self,
        node_id: TypeNodeId,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        children: TypeVariants,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TypeDecl> {
        let db = self.db();
        let mut parser = self.expr_parser(node_id, None, AllowSelfType::True, AllowSelfValue::True);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.parse()?;
        Ok(EnumTypeDecl::new(db, node_id, implicit_parameters, parser.finish()).into())
    }
}
