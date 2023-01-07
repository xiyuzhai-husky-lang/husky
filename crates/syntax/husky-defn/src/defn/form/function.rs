use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FunctionDecl,
    pub expr_sheet: ExprSheet,
    pub body: DefnResult<ExprIdx>,
}
