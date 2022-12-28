use crate::*;

#[salsa::tracked(jar = DeclJar)]
pub struct FeatureDecl {
    #[id]
    pub path: FormPath,
}
