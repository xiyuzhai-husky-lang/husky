use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub path: TraitPath,
    pub decl: TraitDecl,
    pub expr_sheet: ExprSheet,
}
