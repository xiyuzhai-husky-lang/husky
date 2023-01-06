use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FunctionDecl,
    pub variable_sheet: VariableSheet,
    pub expr_sheet: ExprSheet,
    pub body: DefnResult<ExprIdx>,
}
