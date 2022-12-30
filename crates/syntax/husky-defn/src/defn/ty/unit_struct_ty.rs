use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct UnitStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: UnitStructTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
