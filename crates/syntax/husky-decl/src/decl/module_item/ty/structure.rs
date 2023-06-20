use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct StructureTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl StructureTypeNodeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::implicit_parameters)
        //     .unwrap_or(&[])
    }
}

impl<'a> DeclParseContext<'a> {
    pub(super) fn parse_structure_ty_node_decl(
        &self,
        node_path: TypeNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TypeNodeDecl {
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::True);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.try_parse_optional();
        StructureTypeNodeDecl::new(
            self.db(),
            node_path,
            ast_idx,
            implicit_parameters,
            parser.finish(),
        )
        .into()
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct StructureTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}

impl StructureTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: StructureTypeNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(StructureTypeDecl::new(
            db,
            path,
            implicit_parameters,
            expr_region,
        ))
    }
}
