use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct MorphismDefn {
    #[id]
    pub path: FormPath,
    pub expr_sheet: ExprSheet,
    pub decl: MorphismDecl,
}
