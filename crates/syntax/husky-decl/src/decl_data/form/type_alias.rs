use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct TypeAliasDecl {
    pub entity_path: EntityPath,
}
