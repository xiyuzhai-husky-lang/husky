use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct ValueDefn {
    #[id]
    pub path: FormPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: ConstantDecl,
}
