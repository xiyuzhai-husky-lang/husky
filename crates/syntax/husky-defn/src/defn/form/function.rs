use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FunctionDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
