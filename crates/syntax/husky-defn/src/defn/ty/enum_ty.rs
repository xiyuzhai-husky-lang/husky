use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct EnumTypeDefn {
    #[id]
    pub module_item_path: ModuleItemPath,
    pub decl: EnumTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
