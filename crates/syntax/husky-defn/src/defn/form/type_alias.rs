use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAliasDefn {
    #[id]
    pub path: FormPath,
    pub decl: TypeAliasDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
