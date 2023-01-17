use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct MorphismDefn {
    #[id]
    pub path: FormPath,
    pub expr_page: ExprPage,
    pub decl: MorphismDecl,
}
