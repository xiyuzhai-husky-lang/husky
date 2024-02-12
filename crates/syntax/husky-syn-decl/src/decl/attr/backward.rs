use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct BackwardAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub differential_token: IdentRegionalToken,
    #[return_ref]
    pub lpar_token: SynNodeDeclResult<LparRegionalToken>,
    // todo: derivatives
    #[return_ref]
    pub rpar_token: SynNodeDeclResult<RparRegionalToken>,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

/// # getters

impl BackwardAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct BackwardAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    // todo: derivatives
    pub syn_expr_region: SynExprRegion,
}
