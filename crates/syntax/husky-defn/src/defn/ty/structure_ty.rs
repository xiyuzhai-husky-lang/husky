use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: StructureTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
