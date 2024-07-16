use super::*;
use husky_entity_tree::node::attr::AttrSynNodePath;
use husky_syn_expr::syndicates::dep::DepSyndicate;
use parsec::PunctuatedSmallList;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct DepsAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub deps_token: IdentRegionalToken,
    #[return_ref]
    pub lpar_token: SynNodeDeclResult<LparRegionalToken>,
    #[return_ref]
    pub deps: SynNodeDeclResult<
        PunctuatedSmallList<DepSyndicate, CommaRegionalToken, SynNodeDeclError, true, 8>,
    >,
    #[return_ref]
    pub rpar_token: SynNodeDeclResult<RparRegionalToken>,
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
        let deps_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let lpar_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedLeftDelimiterInDerive);
        let deps = parser.try_parse();
        let rpar_token =
            parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedRightDelimiterInDerive);
        Self::new_inner(
            db,
            syn_node_path,
            pound_token,
            deps_token,
            lpar_token,
            deps,
            rpar_token,
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
    #[return_ref]
    pub deps: SmallVec<[DepSyndicate; 4]>,
    pub syn_expr_region: SynExprRegion,
}

impl DepsAttrSynDecl {
    pub(super) fn from_node(
        path: AttrItemPath,
        syn_node_decl: DepsAttrSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        let deps = SmallVec::from(syn_node_decl.deps(db).as_ref()?.elements());
        Ok(Self::new(db, path, deps, syn_expr_region))
    }
}
