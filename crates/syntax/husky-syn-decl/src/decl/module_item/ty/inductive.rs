use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct InductiveTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub expr_region: SynExprRegion,
}

impl InductiveTypeSynNodeDecl {
    pub fn generic_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [GenericParameterDecl] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::generic_parameters)
        //     .unwrap_or(&[])
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
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
        syn_node_path: TypeSynNodePath,
        token_group_idx: TokenGroupIdx,
        variants: TypeVariants,
        saved_stream_state: TokenStreamState,
    ) -> InductiveTypeSynNodeDecl {
        let mut parser = self.expr_parser(
            syn_node_path,
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameter_decl_list = ctx.try_parse_option();
        InductiveTypeSynNodeDecl::new(
            self.db(),
            syn_node_path,
            ast_idx,
            implicit_parameter_decl_list,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct InductiveTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: SynExprRegion,
}

impl InductiveTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: InductiveTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = syn_node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = syn_node_decl.expr_region(db);
        Ok(Self::new(db, path, generic_parameters, expr_region))
    }
}
