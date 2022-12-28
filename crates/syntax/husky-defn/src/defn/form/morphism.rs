use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct MorphismDefn {
    #[id]
    pub path: FormPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: MorphismDecl,
}
