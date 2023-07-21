use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitNodeDecl {
    #[id]
    pub syn_node_path: TraitSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub expr_region: SynExprRegion,
}

impl TraitNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

impl HasNodeDecl for TraitSynNodePath {
    type NodeDecl = TraitNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        trai_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_node_decl(db: &dyn DeclDb, syn_node_path: TraitSynNodePath) -> TraitNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.module_path(db));
    parser.parse_trai_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_node_decl(&self, syn_node_path: TraitSynNodePath) -> TraitNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_trai_decl_aux(
                ast_idx,
                syn_node_path,
                token_group_idx,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_trai_decl_aux(
        &self,
        ast_idx: AstIdx,
        id: TraitSynNodePath,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> TraitNodeDecl {
        let mut parser = self.expr_parser(id, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let generic_parameters = ctx.try_parse_option();
        TraitNodeDecl::new(self.db(), id, ast_idx, generic_parameters, parser.finish())
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar, constructor = new)]
pub struct TraitDecl {
    #[id]
    pub path: TraitPath,
    pub ast_idx: AstIdx,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: SynExprRegion,
}

impl TraitDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitPath,
        node_decl: TraitNodeDecl,
    ) -> DeclResult<TraitDecl> {
        let ast_idx = node_decl.ast_idx(db);
        let generic_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(TraitDecl::new(
            db,
            path,
            ast_idx,
            generic_parameters,
            expr_region,
        ))
    }
}

impl HasDecl for TraitPath {
    type Decl = TraitDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        trai_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_decl(db: &dyn DeclDb, path: TraitPath) -> DeclResult<TraitDecl> {
    let node_decl = path.syn_node_path(db).node_decl(db);
    TraitDecl::from_node_decl(db, path, node_decl)
}
