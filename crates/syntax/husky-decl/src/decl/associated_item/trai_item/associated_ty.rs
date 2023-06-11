use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitAssociatedTypeRawDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct TraitAssociatedTypeDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}

impl<'a> DeclParseContext<'a> {}
