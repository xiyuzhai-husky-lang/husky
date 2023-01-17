use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeAssociatedValueDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_page: ExprPage,
    pub decl: TypeAssociatedValueDecl,
}
