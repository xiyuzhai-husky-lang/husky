use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedFunctionDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TraitForTypeAssociatedFunctionDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_function_defn(
    _db: &dyn DefnDb,
    _decl: TraitForTypeAssociatedFunctionDecl,
) -> TraitForTypeAssociatedFunctionDefn {
    todo!()
}
