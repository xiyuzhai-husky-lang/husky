use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitMethodDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub decl: TypeAsTraitMethodDecl,
    pub expr_page: ExprPage,
    pub body: DefnResult<ExprIdx>,
}
