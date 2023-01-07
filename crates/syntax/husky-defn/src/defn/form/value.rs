use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct ValueDefn {
    #[id]
    pub path: FormPath,
    pub expr_sheet: ModuleItemDefnExprSheet,
    pub decl: ValueDecl,
}
