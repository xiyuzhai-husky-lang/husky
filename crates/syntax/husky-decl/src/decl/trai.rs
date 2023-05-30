use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitDecl {
    #[id]
    pub path: TraitPath,
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

impl HasDecl for TraitPath {
    type Decl = TraitDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        trai_decl(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar,return_ref)]
pub(crate) fn trai_decl(db: &dyn DeclDb, path: TraitPath) -> DeclResult<TraitDecl> {
    let parser = DeclParseContext::new(db, path.module_path(db))?;
    parser.parse_trai_decl(path)
}

impl<'a> DeclParseContext<'a> {
    fn parse_trai_decl(&self, path: TraitPath) -> DeclResult<TraitDecl> {
        let ast_idx = self.resolve_module_item_ast_idx(path);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_trai_decl_aux(ast_idx, path, token_group_idx, saved_stream_state),
            _ => unreachable!(),
        }
    }

    fn parse_trai_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: TraitPath,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> DeclResult<TraitDecl> {
        let mut parser = self.expr_parser(
            DeclRegionPath::Entity(path.into()),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let implicit_parameters = ctx.parse()?;
        Ok(TraitDecl::new(
            self.db(),
            path,
            ast_idx,
            parser.finish(),
            implicit_parameters,
        ))
    }
}
