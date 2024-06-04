use super::*;
use husky_entity_tree::node::attr::AttrSynNodePath;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct DepsAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub marker_token: IdentRegionalToken,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

/// # constructor

impl DepsAttrSynNodeDecl {
    pub(super) fn new(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> Self {
        let parser_factory = ItemSynNodeDeclParser::new(db, syn_node_path.into());
        let mut parser = parser_factory.expr_parser(
            syn_node_path
                .parent_syn_node_path(db)
                .syn_node_decl(db)
                .syn_expr_region(db),
            AllowSelfType::False,
            AllowSelfValue::False,
            None,
        );
        let pound_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let marker_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        Self::new_inner(
            db,
            syn_node_path,
            pound_token,
            marker_token,
            parser.finish(),
        )
    }
}

/// # getters

impl DepsAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

#[salsa::tracked]
pub struct DepsAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    // todo: Depss
    pub syn_expr_region: SynExprRegion,
}

impl DepsAttrSynDecl {
    pub(super) fn from_node(
        path: AttrItemPath,
        syn_node_decl: DepsAttrSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, syn_expr_region))
    }
}
