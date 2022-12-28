use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TypeMemoDefn {
    #[id]
    pub entity_path: EntityPath,
    #[return_ref]
    pub expr_sheet: ExprSheet,
    pub decl: TypeMemoDecl,
}
