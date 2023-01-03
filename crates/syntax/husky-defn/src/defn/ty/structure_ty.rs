use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct StructureTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: StructureTypeDecl,
    pub expr_sheet: ExprSheet,
}
