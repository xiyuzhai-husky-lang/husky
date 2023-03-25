use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAsTraitAssociatedTypeDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TypeAsTraitAssociatedTypeDecl,
}
