use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct InductiveTypeNodeDecl {
    #[id]
    pub node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub expr_region: SynExprRegion,
}

impl InductiveTypeNodeDecl {
    pub fn generic_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::generic_parameters)
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
    pub(super) fn parse_inductive_ty_node_decl(
        &self,
        ast_idx: AstIdx,
        node_path: TypeSynNodePath,
        token_group_idx: TokenGroupIdx,
        variants: TypeVariants,
        saved_stream_state: TokenStreamState,
    ) -> InductiveTypeNodeDecl {
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameter_decl_list = ctx.try_parse_option();
        InductiveTypeNodeDecl::new(
            self.db(),
            node_path,
            ast_idx,
            implicit_parameter_decl_list,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct InductiveTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: SynExprRegion,
}

impl InductiveTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: InductiveTypeNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(Self::new(db, path, generic_parameters, expr_region))
    }
}
