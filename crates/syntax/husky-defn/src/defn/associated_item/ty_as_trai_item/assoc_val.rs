use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedValueDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_sheet: ExprSheet,
    pub decl: TypeAsTraitAssociatedValueDecl,
}
