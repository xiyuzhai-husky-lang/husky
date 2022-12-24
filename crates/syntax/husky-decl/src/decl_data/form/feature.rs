use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FeatureDecl {
    pub entity_path: EntityPath,
}
