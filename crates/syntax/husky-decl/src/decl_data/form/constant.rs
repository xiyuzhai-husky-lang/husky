use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct ConstantDecl {
    pub entity_path: EntityPath,
}
