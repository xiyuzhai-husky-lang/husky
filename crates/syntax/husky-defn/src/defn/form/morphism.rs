use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct MorphismDefn {
    #[id]
    pub path: FormPath,
    pub expr_sheet: ModuleItemDefnExprSheet,
    pub decl: MorphismDecl,
}
