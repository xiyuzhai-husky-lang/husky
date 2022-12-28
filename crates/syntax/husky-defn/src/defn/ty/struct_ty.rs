use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct StructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
