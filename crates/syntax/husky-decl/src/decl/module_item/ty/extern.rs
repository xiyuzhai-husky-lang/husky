use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct ExternTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl ExternTypeNodeDecl {
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
    // get declaration from tokens
    pub(super) fn parse_extern_ty_node_decl(
        &self,
        node_path: TypeNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> ExternTypeNodeDecl {
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, Some(saved_stream_state));
        let generic_parameters = ctx.try_parse_option();
        ExternTypeNodeDecl::new(
            self.db(),
            node_path,
            ast_idx,
            generic_parameters,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct ExternTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}

impl ExternTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: ExternTypeNodeDecl,
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

#[test]
fn extern_ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain);
    let array_ty_decl = entity_path_menu.array_ty_path().decl(&db).unwrap();
    assert_eq!(array_ty_decl.generic_parameters(&db).len(), 2);
}
