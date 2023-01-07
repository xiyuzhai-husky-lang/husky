use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMemoDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_sheet: AssociatedItemDefnExprSheet,
    pub decl: TypeMemoDecl,
}
