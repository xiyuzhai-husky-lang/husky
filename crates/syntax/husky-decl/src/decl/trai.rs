use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TraitDecl {
    #[id]
    pub path: TraitPath,
}
