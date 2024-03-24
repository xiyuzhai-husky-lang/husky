use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct TestAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub test_token: IdentRegionalToken,
    #[return_ref]
    pub lpar_token: SynNodeDeclResult<LparRegionalToken>,
    // todo: Tests
    #[return_ref]
    pub rpar_token: SynNodeDeclResult<RparRegionalToken>,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

/// # getters

impl TestAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TestAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    // todo: Tests
    pub syn_expr_region: SynExprRegion,
}

impl TestAttrSynDecl {
    pub(super) fn from_node(node_decl: TestAttrSynNodeDecl, db: &::salsa::Db) -> DeclResult<Self> {
        todo!()
    }
}
