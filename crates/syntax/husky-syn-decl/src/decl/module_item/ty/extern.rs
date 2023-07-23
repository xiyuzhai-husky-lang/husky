use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct ExternTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub syn_expr_region: SynExprRegion,
}

impl ExternTypeSynNodeDecl {
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
    // get declaration from tokens
    pub(super) fn parse_extern_ty_node_decl(
        &self,
        syn_node_path: TypeSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> ExternTypeSynNodeDecl {
        let mut parser = self.expr_parser(
            syn_node_path,
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let generic_parameters = ctx.try_parse_option();
        ExternTypeSynNodeDecl::new(
            self.db(),
            syn_node_path,
            ast_idx,
            generic_parameters,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct ExternTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl ExternTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: ExternTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = syn_node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, generic_parameters, syn_expr_region))
    }
}

#[test]
fn extern_ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    let array_ty_decl = entity_path_menu.array_ty_path().syn_decl(&db).unwrap();
    assert_eq!(array_ty_decl.generic_parameters(&db).len(), 2);
}
