use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedValSynNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedValSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedValSynDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: SynExprRegion,
}
