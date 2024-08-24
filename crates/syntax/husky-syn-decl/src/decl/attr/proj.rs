use super::*;
use husky_entity_tree::node::attr::AttrSynNodePath;
use husky_syn_expr::syndicates::proj::ProjSyndicate;
use parsec::PunctuatedSmallList;
use vec_like::SmallVecSet;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct ProjAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub projection_token: IdentRegionalToken,
    #[return_ref]
    pub projs: SynNodeDeclResult<
        PunctuatedSmallList<ProjSyndicate, CommaRegionalToken, SynNodeDeclError, true, 8>,
    >,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

/// # constructor

impl ProjAttrSynNodeDecl {
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
        let projection_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let projections = todo!();
        Self::new_inner(
            db,
            syn_node_path,
            pound_token,
            projection_token,
            projections,
            parser.finish(),
        )
    }
}

/// # getters

impl ProjAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

#[salsa::tracked]
pub struct ProjAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    pub projs: SmallVecSet<ProjSyndicate, 2>,
    pub syn_expr_region: SynExprRegion,
}

impl ProjAttrSynDecl {
    pub(super) fn from_node(
        path: AttrItemPath,
        syn_node_decl: ProjAttrSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        let projs =
            SmallVecSet::from_iter(syn_node_decl.projs(db).as_ref()?.elements().iter().copied());
        Ok(Self::new(db, path, projs, syn_expr_region))
    }
}
