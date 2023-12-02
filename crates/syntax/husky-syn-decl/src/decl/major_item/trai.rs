use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitSynNodeDecl {
    #[id]
    pub syn_node_path: TraitSynNodePath,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub syn_expr_region: SynExprRegion,
}

impl TraitSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
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

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        trai_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: TraitSynNodePath,
) -> TraitSynNodeDecl {
    DeclParser::new(db, syn_node_path.into()).parse_trai_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_syn_node_decl(&self, syn_node_path: TraitSynNodePath) -> TraitSynNodeDecl {
        let mut parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        let template_parameters = parser.try_parse_option();
        TraitSynNodeDecl::new(
            self.db(),
            syn_node_path,
            template_parameters,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct TraitSynDecl {
    #[id]
    pub path: TraitPath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub syn_expr_region: SynExprRegion,
}

impl TraitSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: TraitPath,
        syn_node_decl: TraitSynNodeDecl,
    ) -> DeclResult<TraitSynDecl> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(TraitSynDecl::new(
            db,
            path,
            template_parameters,
            syn_expr_region,
        ))
    }
}

impl HasSynDecl for TraitPath {
    type Decl = TraitSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        trai_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_syn_decl(db: &::salsa::Db, path: TraitPath) -> DeclResult<TraitSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitSynDecl::from_node_decl(db, path, syn_node_decl)
}
