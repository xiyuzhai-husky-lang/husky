use super::*;

#[salsa::tracked]
pub struct TraitForTypeAssocValSynNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub syn_expr_region: SynExprRegion,
}

/// # getters
impl TraitForTypeAssocValSynNodeDecl {
    pub fn errors(self, _db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

#[salsa::tracked]
pub struct TraitForTypeAssocValSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub syn_expr_region: SynExprRegion,
}
