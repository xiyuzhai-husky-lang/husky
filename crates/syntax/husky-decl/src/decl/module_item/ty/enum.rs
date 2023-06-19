use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct EnumTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl EnumTypeNodeDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDeclPattern] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::implicit_parameters)
        //     .unwrap_or(&[])
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_enum_ty_node_decl(
        &self,
        node_path: TypeNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        children: TypeVariants,
        saved_stream_state: TokenStreamState,
    ) -> EnumTypeNodeDecl {
        let db = self.db();
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::True);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.try_parse_optional();
        EnumTypeNodeDecl::new(db, node_path, ast_idx, implicit_parameters, parser.finish())
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct EnumTypeDecl {
    #[id]
    pub node_path: TypeNodePath,
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
