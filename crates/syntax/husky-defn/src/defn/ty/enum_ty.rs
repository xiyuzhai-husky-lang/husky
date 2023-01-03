use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct EnumTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeDecl,
    pub expr_sheet: ExprSheet,
}
