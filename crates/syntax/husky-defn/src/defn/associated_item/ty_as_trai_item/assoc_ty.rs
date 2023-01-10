use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedTypeDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_sheet: ExprSheet,
    pub decl: TypeAsTraitAssociatedTypeDecl,
}
