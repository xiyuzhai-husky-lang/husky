use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FeatureDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: FeatureDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
