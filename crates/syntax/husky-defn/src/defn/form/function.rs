use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: FunctionDecl,
}
