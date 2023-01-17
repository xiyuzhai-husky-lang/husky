use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FunctionDecl,
    pub expr_page: ExprPage,
    pub body: DefnResult<ExprIdx>,
}
