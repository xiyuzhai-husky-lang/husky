use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAssociatedFunctionDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TypeAssociatedFunctionDecl,
}
