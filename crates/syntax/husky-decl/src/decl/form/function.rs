use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FunctionDecl {
    pub entity_path: EntityPath,
}
