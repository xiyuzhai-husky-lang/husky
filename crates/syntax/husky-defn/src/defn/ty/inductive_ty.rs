use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub module_item_path: ConnectedModuleItemPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    pub decl: InductiveTypeDecl,
}
