use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TraitMethodDefn {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: MethodDecl,
}
