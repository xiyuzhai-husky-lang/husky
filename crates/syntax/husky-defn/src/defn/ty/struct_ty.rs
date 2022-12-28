use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct StructTypeDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: StructTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
