use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct InductiveTypeDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: InductiveTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
