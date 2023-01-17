use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedTypeDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_page: ExprPage,
    pub decl: TypeAsTraitAssociatedTypeDecl,
}
