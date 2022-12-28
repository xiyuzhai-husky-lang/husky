use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TraitMethodDecl {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_arena: ExprArena,
}
