use super::*;

#[salsa::tracked(jar = DeclJar)]
pub struct AlienTypeDecl {
    pub path: TypePath,
}
