use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct InductiveTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    pub syn_expr_region: SynExprRegion,
}

impl InductiveTypeSynNodeDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        todo!()
        // self.template_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::template_parameters)
        //     .unwrap_or(&[])
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

impl<'a> DeclParserFactory<'a> {
    pub(super) fn parse_inductive_ty_node_decl(
        &self,
        ast_idx: AstIdx,
        syn_node_path: TypeSynNodePath,
        token_group_idx: TokenGroupIdx,
        variants: TypeVariants,
        saved_stream_state: TokenStreamState,
    ) -> InductiveTypeSynNodeDecl {
        let mut parser = self.parser(
            syn_node_path,
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
            None,
            token_group_idx,
            Some(saved_stream_state),
        );
        let template_parameter_decl_list = parser.try_parse_option();
        InductiveTypeSynNodeDecl::new(
            self.db(),
            syn_node_path,
            ast_idx,
            template_parameter_decl_list,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct InductiveTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl InductiveTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: InductiveTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, template_parameters, syn_expr_region))
    }
}
