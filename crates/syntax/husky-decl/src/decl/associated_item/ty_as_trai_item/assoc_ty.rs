use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAsTraitAssociatedTypeDecl {
    #[id]
    pub entity_path: EntityPath,
    pub ast_idx: AstIdx,
    pub expr_region: ExprRegion,
}
