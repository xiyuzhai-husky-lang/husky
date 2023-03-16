use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FnDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
