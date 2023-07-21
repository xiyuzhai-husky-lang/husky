use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedValNodeDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedValNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        // ad hoc
        Default::default()
    }
}

impl<'a> DeclParser<'a> {}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitForTypeAssociatedValDecl {
    #[id]
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: SynExprRegion,
}
