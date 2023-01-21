use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedValueDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TypeAsTraitAssociatedValueDecl,
}
