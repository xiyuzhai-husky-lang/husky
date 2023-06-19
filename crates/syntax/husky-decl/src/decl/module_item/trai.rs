use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitNodeDecl {
    #[id]
    pub node_path: TraitNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl TraitNodeDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }
}

impl HasNodeDecl for TraitNodePath {
    type NodeDecl = TraitNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        todo!()
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitDecl {
    #[id]
    pub node_path: TraitNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: Option<ImplicitParameterDeclList>,
}

impl TraitDecl {
    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self.implicit_parameter_decl_list(db) {
            Some(list) => list.implicit_parameters(),
            None => &[],
        }
    }
}

impl HasDecl for TraitNodePath {
    type Decl = TraitDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        trai_decl(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar,return_ref)]
pub(crate) fn trai_decl(db: &dyn DeclDb, id: TraitNodePath) -> DeclResult<TraitDecl> {
    let parser = DeclParseContext::new(db, id.module_path(db))?;
    parser.parse_trai_decl(id)
}

impl<'a> DeclParseContext<'a> {
    fn parse_trai_decl(&self, node_path: TraitNodePath) -> DeclResult<TraitDecl> {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_trai_decl_aux(ast_idx, node_path, token_group_idx, saved_stream_state),
            _ => unreachable!(),
        }
    }

    fn parse_trai_decl_aux(
        &self,
        ast_idx: AstIdx,
        id: TraitNodePath,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TraitDecl> {
        let mut parser = self.expr_parser(id, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.parse()?;
        Ok(TraitDecl::new(
            self.db(),
            id,
            ast_idx,
            parser.finish(),
            implicit_parameters,
        ))
    }
}

impl HasDecl for TraitPath {
    type Decl = TraitDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        self.node_path(db).decl(db)
    }
}
