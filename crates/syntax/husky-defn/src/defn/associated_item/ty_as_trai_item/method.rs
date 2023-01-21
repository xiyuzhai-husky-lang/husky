use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitMethodDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub decl: TypeAsTraitMethodDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}
