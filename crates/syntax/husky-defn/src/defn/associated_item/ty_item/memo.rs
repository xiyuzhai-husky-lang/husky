use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMemoDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMemoDecl,
    pub expr_page: ExprPage,
    pub body: DefnResult<ExprIdx>,
}
