use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: FunctionDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
