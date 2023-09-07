use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitSynNodeDecl {
    #[id]
    pub syn_node_path: TraitSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

impl HasSynNodeDecl for TraitSynNodePath {
    type NodeDecl = TraitSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        trai_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: TraitSynNodePath,
) -> TraitSynNodeDecl {
    let parser = DeclParserFactory::new(db, syn_node_path.module_path(db));
    parser.parse_trai_syn_node_decl(syn_node_path)
}

impl<'a> DeclParserFactory<'a> {
    fn parse_trai_syn_node_decl(&self, syn_node_path: TraitSynNodePath) -> TraitSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Identifiable {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_trai_syn_decl_aux(
                ast_idx,
                syn_node_path,
                token_group_idx,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_trai_syn_decl_aux(
        &self,
        ast_idx: AstIdx,
        id: TraitSynNodePath,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitSynNodeDecl {
        let mut parser = self.expr_parser(id, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let template_parameters = ctx.try_parse_option();
        TraitSynNodeDecl::new(self.db(), id, ast_idx, template_parameters, parser.finish())
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct TraitSynDecl {
    #[id]
    pub path: TraitPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl TraitSynDecl {
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TraitPath,
        syn_node_decl: TraitSynNodeDecl,
    ) -> DeclResult<TraitSynDecl> {
        let ast_idx = syn_node_decl.ast_idx(db);
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(TraitSynDecl::new(
            db,
            path,
            ast_idx,
            template_parameters,
            syn_expr_region,
        ))
    }
}

impl HasSynDecl for TraitPath {
    type Decl = TraitSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        trai_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_syn_decl(db: &dyn SynDeclDb, path: TraitPath) -> DeclResult<TraitSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitSynDecl::from_node_decl(db, path, syn_node_decl)
}
