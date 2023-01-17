use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAssociatedFunctionDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_page: ExprPage,
    pub decl: TypeAssociatedFunctionDecl,
}
