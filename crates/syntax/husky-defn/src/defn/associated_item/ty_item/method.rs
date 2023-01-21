use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMethodDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMethodDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
