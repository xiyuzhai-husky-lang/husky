use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedFunctionDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_sheet: ExprSheet,
    pub decl: TypeAsTraitAssociatedFunctionDecl,
}
