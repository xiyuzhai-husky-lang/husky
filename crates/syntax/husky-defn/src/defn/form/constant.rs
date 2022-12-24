use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct ConstantDefn {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: ConstantDecl,
}
