use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMethodDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMethodDecl,
    pub expr_sheet: ExprSheet,
    pub body: DefnResult<ExprIdx>,
}
