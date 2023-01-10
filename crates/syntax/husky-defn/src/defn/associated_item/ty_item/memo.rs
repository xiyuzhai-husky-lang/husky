use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMemoDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMemoDecl,
    pub expr_sheet: ExprSheet,
    pub body: DefnResult<ExprIdx>,
}
