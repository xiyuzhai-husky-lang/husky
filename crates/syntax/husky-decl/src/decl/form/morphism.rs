use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct MorphismDecl {
    #[id]
    pub path: FormPath,
}
