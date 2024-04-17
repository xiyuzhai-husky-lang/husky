use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct EffectAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub effect_token: IdentRegionalToken,
    #[return_ref]
    pub lpar_token: SynNodeDeclResult<LparRegionalToken>,
    // todo: effects
    #[return_ref]
    pub rpar_token: SynNodeDeclResult<RparRegionalToken>,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

/// # constructor

impl EffectAttrSynNodeDecl {
    pub(super) fn new(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> Self {
        todo!()
    }
}

/// # getters

impl EffectAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct EffectAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    // todo: effects
    pub syn_expr_region: SynExprRegion,
}
