use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAsTraitAssociatedFunctionDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TypeAsTraitAssociatedFunctionDecl,
}
