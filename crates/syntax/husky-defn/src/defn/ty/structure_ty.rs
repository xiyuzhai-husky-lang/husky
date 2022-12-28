use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
