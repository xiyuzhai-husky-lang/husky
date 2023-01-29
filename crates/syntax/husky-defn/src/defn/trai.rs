use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitDefn {
    #[id]
    pub path: TraitPath,
    pub decl: TraitDecl,
}
