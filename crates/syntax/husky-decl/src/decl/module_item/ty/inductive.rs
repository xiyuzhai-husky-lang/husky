use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct InductiveTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl InductiveTypeNodeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::implicit_parameters)
        //     .unwrap_or(&[])
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_inductive_ty_node_decl(
        &self,
        ast_idx: AstIdx,
        node_path: TypeNodePath,
        token_group_idx: TokenGroupIdx,
        variants: TypeVariants,
        saved_stream_state: TokenStreamState,
    ) -> InductiveTypeNodeDecl {
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameter_decl_list = ctx.try_parse_optional();
        InductiveTypeNodeDecl::new(
            self.db(),
            node_path,
            ast_idx,
            implicit_parameter_decl_list,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct InductiveTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}

impl InductiveTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: InductiveTypeNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
