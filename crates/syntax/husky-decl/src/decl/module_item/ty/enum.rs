use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct EnumTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl EnumTypeNodeDecl {
    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::implicit_parameters)
        //     .unwrap_or(&[])
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

impl<'a> DeclParser<'a> {
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
    pub path: TypePath,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}

impl EnumTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: EnumTypeNodeDecl,
    ) -> DeclResult<Self> {
        let implicit_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.implicit_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(Self::new(db, path, implicit_parameters, expr_region))
    }
}
