use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAsTraitAssociatedTypeDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TypeAsTraitAssociatedTypeDecl,
}
