use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: RecordTypeDecl,
}
