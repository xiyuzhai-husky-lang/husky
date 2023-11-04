use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct ExternTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub syn_expr_region: SynExprRegion,
}

impl ExternTypeSynNodeDecl {
    pub fn template_parameters<'a>(
        self,
        db: &'a dyn SynDeclDb,
    ) -> &'a [SynTemplateParameterSyndicate] {
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

impl<'a> DeclParser<'a, TypeSynNodePath> {
    // get declaration from tokens
    pub(super) fn parse_extern_ty_node_decl(&self) -> ExternTypeSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        let template_parameters = parser.try_parse_option();
        ExternTypeSynNodeDecl::new(
            self.db(),
            self.syn_node_path(),
            template_parameters,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct ExternTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: TemplateParameterSyndicates,
    pub syn_expr_region: SynExprRegion,
}

impl ExternTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: ExternTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.syn_template_parameter_obelisks().to_smallvec())
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, template_parameters, syn_expr_region))
    }
}

#[test]
fn extern_ty_decl_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let item_path_menu = db.item_path_menu(toolchain);
    let array_ty_decl = item_path_menu.array_ty_path().syn_decl(&db).unwrap();
    assert_eq!(array_ty_decl.template_parameters(&db).len(), 2);
}
