use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FormFnDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
