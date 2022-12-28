use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: TraitDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
