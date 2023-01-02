use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct ValueDefn {
    #[id]
    pub path: FormPath,
    #[return_ref]
    pub expr_sheet: ExprSheet,
    pub decl: ValueDecl,
}
