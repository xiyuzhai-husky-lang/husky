use crate::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitDecl {
    #[id]
    pub path: TraitPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: DeclExprResult<Option<ImplicitParameterDeclList>>,
}

impl TraitDecl {
    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self.implicit_parameter_decl_list(db).as_ref()? {
            Some(list) => list.implicit_parameters(),
            None => Ok(&[]),
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
                ref body,
                saved_stream_state,
                ..
            } => self.parse_trai_decl_aux(ast_idx, path, token_group_idx, body, saved_stream_state),
            _ => unreachable!(),
        }
    }
}
