use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct EnumTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    template_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub syn_expr_region: SynExprRegion,
}

impl EnumTypeSynNodeDecl {
    pub fn template_parameters(self, db: &dyn SynDeclDb) -> &[TemplateParameterDecl] {
        todo!()
        // self.template_parameter_decl_list(db)
        //     .as_ref()
        //     .map(ImplicitParameterDeclList::template_parameters)
        //     .unwrap_or(&[])
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

impl<'a> DeclParser<'a> {
    pub(super) fn parse_enum_ty_node_decl(
        &self,
        syn_node_path: TypeSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        children: TypeVariants,
        saved_stream_state: TokenStreamState,
    ) -> EnumTypeSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            syn_node_path,
            None,
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let template_parameters = ctx.try_parse_option();
        EnumTypeSynNodeDecl::new(
            db,
            syn_node_path,
            ast_idx,
            template_parameters,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct EnumTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl EnumTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: EnumTypeSynNodeDecl,
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
