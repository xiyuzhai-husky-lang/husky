use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeAssociatedValueDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub expr_region: ExprRegion,
    pub decl: TraitForTypeAssociatedValueDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_associated_value_defn(
    _db: &dyn DefnDb,
    _decl: TraitForTypeAssociatedValueDecl,
) -> TraitForTypeAssociatedValueDefn {
    todo!()
}
