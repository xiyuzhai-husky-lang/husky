use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMethodDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMethodDecl,
    pub expr_page: ExprPage,
    pub body: DefnResult<ExprIdx>,
}
