use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TraitAssociatedValueDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_sheet: AssociatedItemDefnExprSheet,
    pub decl: TraitAssociatedValueDecl,
}
