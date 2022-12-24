use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct MorphismDefn {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: MorphismDecl,
}
