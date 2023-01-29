use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeMemoDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMemoDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
