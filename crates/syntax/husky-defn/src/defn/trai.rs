use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub path: TraitPath,
    pub decl: TraitDecl,
    #[return_ref]
    pub expr_sheet: ExprSheet,
}
