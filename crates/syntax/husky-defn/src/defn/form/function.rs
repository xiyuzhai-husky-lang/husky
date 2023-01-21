use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FunctionDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
