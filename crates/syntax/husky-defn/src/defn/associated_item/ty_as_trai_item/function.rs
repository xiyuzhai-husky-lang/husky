use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAsTraitAssociatedFunctionDefn {
    #[id]
    pub path: Option<TypeAsTraitItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TypeAsTraitAssociatedFunctionDecl,
}
