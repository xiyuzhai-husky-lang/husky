use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAliasDefn {
    #[id]
    pub path: FormPath,
    pub decl: TypeAliasDecl,
    pub expr_sheet: ModuleItemDefnExprSheet,
}
