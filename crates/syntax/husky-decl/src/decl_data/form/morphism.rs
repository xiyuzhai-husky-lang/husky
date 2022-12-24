use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct MorphismDecl {
    pub entity_path: EntityPath,
}
