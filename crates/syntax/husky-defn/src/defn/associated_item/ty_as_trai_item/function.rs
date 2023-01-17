use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedFunctionDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_page: ExprPage,
    pub decl: TypeAsTraitAssociatedFunctionDecl,
}
