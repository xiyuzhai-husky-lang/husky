use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMethodDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_sheet: AssociatedItemDefnExprSheet,
    pub decl: TypeMethodDecl,
}
