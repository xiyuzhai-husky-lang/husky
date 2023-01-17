use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedValueDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_page: ExprPage,
    pub decl: TypeAsTraitAssociatedValueDecl,
}
