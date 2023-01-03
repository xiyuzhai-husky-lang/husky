use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct RecordTypeDefn {
    #[id]
    pub path: TypePath,
    pub expr_sheet: ExprSheet,
    pub decl: RecordTypeDecl,
}
