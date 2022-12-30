use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct MorphismDefn {
    #[id]
    pub path: FormPath,
    #[return_ref]
    pub expr_sheet: ExprSheet,
    pub decl: MorphismDecl,
}
