use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedValSynNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub syn_expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedValSynNodeDecl {
    pub fn errors(self, _db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedValSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub syn_expr_region: SynExprRegion,
}
