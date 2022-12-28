use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAliasDecl {
    #[id]
    pub path: FormPath,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
