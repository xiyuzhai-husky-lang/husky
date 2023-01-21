use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedFunctionDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TypeAsTraitAssociatedFunctionDecl,
}
