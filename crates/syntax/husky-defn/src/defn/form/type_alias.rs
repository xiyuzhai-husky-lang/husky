use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct AliasTypeDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: AliasTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
