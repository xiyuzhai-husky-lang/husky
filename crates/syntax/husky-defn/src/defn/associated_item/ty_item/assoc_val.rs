use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAssociatedValueDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_sheet: ExprSheet,
    pub decl: TypeAssociatedValueDecl,
}
